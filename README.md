# Applying "app-template" on [iotxxx-nrf52832dk](http://doc.iotxx.com/NRF52832DK) 

- hardware
    - NRF52832DK
    - Jlink-lite

![image](https://user-images.githubusercontent.com/35317930/139210322-1b999e2b-e52e-4253-aeb4-e83ac885b45d.png)


# History

## hello world (done)
```console
➜  e73bt git:(main) ✗ cargo rb hello
    Finished dev [optimized + debuginfo] target(s) in 0.01s
     Running `probe-run --chip nRF52832_xxAA target/thumbv7em-none-eabihf/debug/hello`
(HOST) INFO  flashing program (6.78 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
0 INFO  Hello, world!
└─ hello::__cortex_m_rt_main @ src/bin/hello.rs:8
────────────────────────────────────────────────────────────────────────────────
```

# reference
- https://github.com/knurling-rs/app-template

