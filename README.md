# EnerLedger: Red Descentralizada de Medición Energética

EnerLedger es una infraestructura DePIN (Decentralized Physical Infrastructure Network) construida sobre Solana, diseñada para el registro inmutable de mediciones eléctricas y la tokenización de métricas de eficiencia energética.

## Problema y Solución
El proyecto ataca directamente la manipulación de medidores, el robo de energía y la falta de trazabilidad en sistemas centralizados (como los operados por empresas como CFE), proporcionando una capa de confianza inmutable mediante blockchain.

## Funcionalidades Técnicas (CRUD + PDA)
Este contrato inteligente implementa:
- **CREATE**: Inicialización de cuentas de medidores utilizando **PDAs** (Program Derived Addresses) únicos, derivados del ID físico del medidor para garantizar aislamiento y seguridad.
- **READ**: Consulta de métricas de consumo registradas en la blockchain.
- **UPDATE**: Registro inmutable de nuevas lecturas de energía, permitiendo el historial auditado.
- **DELETE**: Cierre seguro de cuentas para la gestión de recursos de la red.

## Arquitectura DePIN
Cada medidor inteligente actúa como un nodo que:
1. Registra el consumo.
2. Genera una firma digital.
3. Almacena el hash en Solana para garantizar integridad.

## Cómo usar este proyecto
1. **Requisitos**: Tener instalado `Rust`, `Solana CLI` y `Anchor CLI`.
2. **Compilar**: Ejecuta `anchor build` en la terminal dentro de la carpeta raíz.
3. **Probar**: Ejecuta `anchor test` para verificar la lógica CRUD + PDA.

## Tecnologías Utilizadas
- **Blockchain**: Solana
- **Framework**: Anchor
- **Lenguaje**: Rust
- **Concepto**: DePIN (Decentralized Physical Infrastructure Networks)
