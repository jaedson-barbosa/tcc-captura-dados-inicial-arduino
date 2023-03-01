use chrono::{DateTime, Utc};
use median::Filter;
use plotters::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Registro {
    #[serde(with = "my_date_format")]
    horario: DateTime<Utc>,
    pressao: i32,
    geracao: i32,
    vazao: i32,
}

mod my_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, "%d/%m/%Y %H:%M:%S")
            .map_err(serde::de::Error::custom)
    }
}

fn main() -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path("./result.csv")?;
    let mut registros: Vec<Registro> = reader.deserialize().map(|v| v.unwrap()).collect();
    let vazao_inicial = registros[0].vazao;
    registros
        .iter_mut()
        .fold(vazao_inicial, |acc, v: &mut Registro| {
            let valor = v.vazao;
            v.vazao = v.vazao - acc; // A vazão é representada pela diferença no contador
            valor
        });
    let horario_inicial = registros[0].horario;
    let horario_final = registros.last().unwrap().horario;

    const LARGURA_FILTRO: usize = 10;
    let plot_grafico = |name: &str, get_y: fn(v: &Registro) -> f64| {
        let mut filtro = Filter::new(LARGURA_FILTRO);
        let data: Vec<(DateTime<_>, f64)> = registros
            .iter()
            .map(|v| (v.horario, filtro.consume(get_y(v))))
            .collect();
        let y_limit = data.iter().map(|v| v.1).fold(f64::NAN, f64::max);

        let file_path = format!("images/{name}.png");
        let root_area = BitMapBackend::new(&file_path, (640, 480)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 24)
            .set_label_area_size(LabelAreaPosition::Bottom, 24)
            .margin(10)
            .build_cartesian_2d(horario_inicial..horario_final, 0.0..y_limit)
            .unwrap();

        ctx.configure_mesh()
            .x_label_formatter(&|x| x.format("%M:%S").to_string())
            .draw()
            .unwrap();

        ctx.draw_series(LineSeries::new(data, &BLACK))
            .unwrap()
            .label(name);
    };

    plot_grafico("Pressão (raw)", |v| v.pressao as f64);
    plot_grafico("Geração (raw)", |v| v.geracao as f64);
    plot_grafico("Vazão (pulsos)", |v| v.vazao as f64);

    const ANALOG_TO_MV: f64 = 1.0 / 4095.0;
    const RELACAO_POTENCIOMETRO_PRESSAO: f64 = 123.0 / 1_800.0;
    const RELACAO_POTENCIOMETRO_GERACAO: f64 = 138.0 / 1_080.0;
    const FATOR_PRESSAO_V: f64 = ANALOG_TO_MV / RELACAO_POTENCIOMETRO_PRESSAO;
    const FATOR_GERACAO_V: f64 = ANALOG_TO_MV / RELACAO_POTENCIOMETRO_GERACAO;
    const FATOR_VAZAO_ML_S: f64 = 10.0 * 1000.0 / 450.0;
    const FATOR_VAZAO_L_MIN: f64 = 10.0 * 60.0 / 450.0;
    plot_grafico("Pressão (V)", |v| FATOR_PRESSAO_V * v.pressao as f64);
    plot_grafico("Geração (V)", |v| FATOR_GERACAO_V * v.geracao as f64);
    plot_grafico("Vazão (mL s)", |v| FATOR_VAZAO_ML_S * v.vazao as f64);
    plot_grafico("Vazão (L min)", |v| FATOR_VAZAO_L_MIN * v.vazao as f64);

    println!("Horarios inicial e final da coleta: {horario_inicial} e {horario_final}");
    Ok(())
}
