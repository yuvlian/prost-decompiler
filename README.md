# prost2proto

Turn back prost output files to .proto

Usage: `python main.py`

NOTE:
- It is recommended to use `rustfmt` like this: `rustfmt --config max_width=2000 your_file.rs`, before running the python script.

- This code assumes the prost output was built with default configuration. You may need to modify the code a little, if the prost output has custom derive macros or whatever. Should be easy.

Features (Anything that isn't here is probably not supported, like gRPC, etc.):
- Proto3
- Capturing oneofs
- Capturing enums
- Capturing messages

Known issues:

- Valid rust enum names being fucked in .proto because of cpp scoping

- Nested enums being fucked, example:
  ```rust
  pub struct PlayerKickOutScNotify {
      #[prost(message, optional, tag = "4")]
      pub black_info: ::core::option::Option<BlackInfo>,
      #[prost(enumeration = "player_kick_out_sc_notify::Hilaijmdkej", tag = "15")]
      pub mglldoifgnd: i32,
  }
  /// Nested message and enum types in `PlayerKickOutScNotify`.
  pub mod player_kick_out_sc_notify {
      #[derive(proto_derive::CmdID, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
      #[repr(i32)]
      pub enum Hilaijmdkej {
          KickSqueezed = 0,
          KickBlack = 1,
          KickChangePwd = 2,
          KickLoginWhiteTimeout = 3,
          KickAceAntiCheater = 4,
          KickByGm = 5,
      }
    ```
    will turn into:
    ```proto
    message PlayerKickOutScNotify {
      BlackInfo black_info = 4;
      player_kick_out_sc_notify::Hilaijmdkej mglldoifgnd = 15;
    }
    ```
    and the enum itself isn't parsed
