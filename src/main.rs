use serde::Deserialize;

#[derive(Deserialize)]
struct Registro {
    horario: String,
    pressao: u16,
    geracao: u16,
    vazao: u16
}

fn main() -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path("../result.csv")?;
    for registro in reader.deserialize() {
        let registro: Registro = registro?;
        println!("{}, {}, {}, {}", registro.horario, registro.pressao, registro.geracao, registro.vazao);
    }
    Ok(())
}

// Continuar leitura de https://plotters-rs.github.io/book/basic/basic_data_plotting.html
