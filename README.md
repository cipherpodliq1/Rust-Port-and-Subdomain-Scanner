# Rust Port and Subdomain Scanner
A simple attack surface scanner written in Rust. Please do not use it in real life scenarios, because it is noisy and most likely it will get picked up by defender's software.

It finds the subdomains of a target, and then scans the ports of those subdomains. For subdomain enumeration , it uses the API provided by [crt.sh](https://crt.sh/) . Endpoint being : *https://crt.sh/?q=%25.[domain.com]&output=json"*

For port scanning , it uses the so-called "TCP-Connect".

It uses the [anyhow](https://docs.rs/anyhow/latest/anyhow/) and [thiserror](https://docs.rs/thiserror/latest/thiserror/) crates for error-handling.

In order to turn our scanner from sequential (slow), to "multi-threaded" (fast), we use the [rayon](https://docs.rs/rayon/latest/rayon/) rust crate. (A data paralelism library, Because thread synchronization is hard)
IN the background, the rayon crate starts a *thread pool* , and dispatches our tasks to it


