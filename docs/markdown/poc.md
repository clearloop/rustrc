Storage
=======

```

? func to impl
? trait and type
? Send - Sync
? why RawValue
? AsRef - &

```
keypair.rs
==========

!? do SecretKey need sign method? / SecretKey or KeyPair

! Don't build 'nightly' features
! Can't impl struct outside crate
! [u8; 64] can not debug
! sign -- about `cannot resolve `_: digest::digest::Digest``

? as_bytes / to_bytes
? use the <T> in ed25519_dalek::SecretKey::generate
? pub(crate)
? #[repr(C)]
? #[derive(Default)]
? StructName(OtherStructName) - Clone?
? Constructor is not visible here due to private fields

+ VAR_LENGTH -> NUMBER;
+ sign -> Keypair &&  verify -> PublicKey

Storage
=======

```

? func to impl
? trait and type
? Send - Sync
? why RawValue
? AsRef - &

```

P2P
===

```
? extend
? futures <default-features - false>
? to_owned() and `move`
? what `.into()` means
```