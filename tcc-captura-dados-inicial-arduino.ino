#define pino_pressao 4
#define pino_turbina 2
#define pino_vazao 5

long contador_acionamentos = 0;

void IRAM_ATTR funcao_ISR()
{
  contador_acionamentos++;
}

void setup() {
  pinMode(pino_pressao, INPUT);
  pinMode(pino_turbina, INPUT);
  pinMode(pino_vazao, INPUT);
  attachInterrupt(pino_vazao, funcao_ISR, RISING);
  Serial.begin(115200);
}

void loop() {
  auto valor_pressao = analogRead(pino_pressao);
  auto valor_turbina = analogRead(pino_turbina);
  Serial.print(valor_pressao);
  Serial.print(", ");
  Serial.print(valor_turbina);
  Serial.print(", ");
  Serial.print(contador_acionamentos);
  Serial.println(";");
  delay(100);
}
