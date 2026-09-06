# Contribuire a TapDeck

Grazie per voler contribuire! 👋 Queste note ti aiutano a iniziare: tutto qui sotto
è pensato per funzionare in pochi minuti.

## Compilare e testare

Serve [Rust](https://rustup.rs) (toolchain stable) e, su Windows, le *Build
Tools* di Visual Studio (workload C++).

```bash
cargo build            # compila (binario in target\\debug\\tapdeck.exe)
cargo run              # avvia l'editor
cargo test             # esegue tutti i test automatici
cargo clippy --all-targets -- -D warnings   # lint: zero warning ammessi
cargo fmt --check      # formattazione (usa `cargo fmt` per correggere)
```

Regole di base prima di aprire una PR:

- `cargo test` **verde** (al momento 39 test).
- `cargo fmt --check` senza differenze.
- `cargo clippy --all-targets -- -D warnings` senza errori.
- Se tocchi la pagina telefono (`assets/`), ricompila e controlla che la PWA
  funzioni: `cargo run -- --serve-only` e apri `http://127.0.0.1:8787`.

Le stesse verifiche girano automaticamente nella **CI GitHub** (`.github/workflows/ci.yml`)
per ogni push e pull request: se la CI è rossa, la PR non può essere accettata.

## Com'è organizzato il codice

| File | Ruolo |
|---|---|
| `src/main.rs` | Avvio: istanza singola, icona finestra, autofill dei percorsi |
| `src/ui.rs` | Editor egui: pagine, griglia, dialoghi dei tasti |
| `src/server.rs` | Server HTTP (tiny_http) + servizio della PWA |
| `src/actions.rs` | Esecuzione delle azioni sul PC (apri/chiudi, senza console) |
| `src/detect.rs` | Riconoscimento automatico dei programmi installati |
| `src/config.rs` | Config JSON, pagine predefinite, migrazioni |
| `src/icons.rs` | Catalogo loghi ufficiali + generazione icone PWA |
| `src/brand.rs` | Disegno del logo TapDeck (usato da `.ico`, finestra e PWA) |
| `assets/` | Pagina telefono: `index.html`, `app.css`, `app.js`, manifest |
| `examples/gen_icon.rs` | Genera `res/icon.ico` per l'eseguibile |

## Aggiungere un programma al riconoscimento automatico

Il riconoscimento vive in `src/detect.rs`, nella tabella `KNOWN_APPS`.

## Aprire una pull request

1. Crea un fork e un branch: `git checkout -b feat/descrizione-breve`.
2. Fai modifiche piccole e focalizzate; scrivi commit chiari in italiano o inglese.
3. Aggiorna i test e il README se la modifica cambia il comportamento visibile.
4. Controlla: `cargo test` + `cargo fmt --check` + `cargo clippy -D warnings`.
5. Apri la PR con una descrizione di *cosa* cambi e *perché*.

## Pubblicare una release

```bash
git tag v0.1.0
git push origin v0.1.0
```

Il workflow `.github/workflows/release.yml` compila, testa e pubblica
automaticamente su **GitHub Releases** un archivio `tapdeck-windows.zip`.

## Licenza

MIT — vedi [LICENSE](LICENSE).
