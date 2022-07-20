# fbot-vss
FIRASim Rust Client


## Executando Exemplos
- Seguidor de Bola: `cargo run --example follow_ball`

---------------------

## Instalando como dependência
Adicionar a dependência no arquivo `Cargo.toml`
```toml
[dependencies]
fbot_rust_client = { git = "https://github.com/vsssfbot/fbot_rust_client" }
```

O codigo a abaixo obtem os valores do robo amarelo de id 0, e as informações da bola
```rust
use fbot_rust_client::fira_protos;
use fbot_rust_client::{ball, yellow_robot};

fn main() {
    let robot = yellow_robot(&0).unwrap();
    let ball = ball();

    println!("Robot: {:?}", robot);
    println!("Ball{:?}", ball);
}
```

Enviando um commando para o FIRASim

```rust
fn main() {

    //...
    
    let commands = fira_protos::Commands {
        robot_commands: vec![
            fira_protos::Command {
                id: 0,
                yellowteam: true,
                wheel_left: 5.0,
                wheel_right: -5.0,
            },
        ]
    };

    fbot_rust_client::send_command(commands)
}
```
