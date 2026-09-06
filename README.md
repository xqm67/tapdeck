# TapDeck — la tua Stream Deck fatta in casa

![Licenza MIT](https://img.shields.io/badge/licenza-MIT-green)
![Linguaggio: Rust](https://img.shields.io/badge/Rust-1.85+-orange)
![Piattaforma](https://img.shields.io/badge/piattaforma-Windows%2010%2F11-blue)
![Telefono](https://img.shields.io/badge/telefono-iPhone%20%2F%20Android-9146FF)
[![CI](https://img.shields.io/github/actions/workflow/status/xqm67/tapdeck/ci.yml?branch=main&label=CI)](https://github.com/xqm67/tapdeck/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/xqm67/tapdeck?include_prereleases&label=release)](https://github.com/xqm67/tapdeck/releases)

Trasforma il tuo **iPhone (anche iPhone 8) o Android** in una **griglia di tasti
remoti** per il PC Windows: ogni tasto ha il **logo originale** (Discord, Twitch,
Spotify, OBS…) e al tocco esegue un'azione **sul PC** — apre un programma, apre un
sito, chiude un'app. Il telefono è solo il telecomando: **una pagina, un tocco,
un'azione**, proprio come una Stream Deck.

Scritto in **Rust**: un solo `.exe`, nessun runtime da installare.

<p align="center">
  <img src="screenshots/editor.png" alt="Editor TapDeck su PC" width="560">
  <br>
  <em>La finestra di configurazione sul PC</em>
</p>

<p align="center">
  <img src="screenshots/telefono.png" alt="Pagina TapDeck sul telefono" width="240">
  <br>
  <em>La griglia di tasti sul telefono (verticale e orizzontale, schermo intero)</em>
</p>

---

## Caratteristiche

- **Griglia di tasti su telefono**: pagine multiple con **swipe laterale** o
  puntini, **verticale e orizzontale**, pulsante **⛶ per il pieno schermo**.
- **Loghi ufficiali**: catalogo di ~80 app (Discord, Twitch, Spotify, OBS, Steam…)
  con icone salvate in locale, più file PNG o URL personalizzati.
- **Riconoscimento automatico dei programmi**: TapDeck trova da solo dove sono
  installati ~90 programmi comuni.
- **Azioni PC**: *apre un programma*, *apre un sito*, *chiude un'app*.
- **Nessuna finestra nera**: nessun terminale che lampeggia, istanza singola.
- **PIN facoltativo** per l'accesso dal telefono.
- **Installazione vera**: `install.ps1` la registra in *Impostazioni → App*.

## Come funziona

1. Il PC esegue TapDeck: finestra di configurazione + un piccolo **server web**
   sulla rete di casa (porta 8787, modificabile).
2. Il telefono apre l'indirizzo mostrato (o inquadra il **QR code** nella finestra).
3. Da Safari (iPhone): *Condividi* → **Aggiungi alla schermata Home**.
4. Tocchi un tasto → l'azione parte **sul PC**.

## Requisiti

- PC **Windows 10/11**
- **Rust** per compilare da sorgente (https://rustup.rs) + *Build Tools* di Visual Studio (workload C++)
- Telefono e PC sulla **stessa rete Wi-Fi**

## Download

L'ultima build pronta all'uso è nella sezione **Releases** di questo repository:
scarica `tapdeck-windows.zip`, estrailo e lancia `install.ps1`.

## Installazione

### Come una vera app (consigliato)

```powershell
cargo build --release
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

Oppure, se scarichi la release, basta eseguire `install.ps1` senza compilare.

Durante l'installazione viene chiesta la **lingua** (Italiano o English).

- Per tutti gli utenti: `-AllUsers`
- Disinstallare: Impostazioni → App → TapDeck → Disinstalla
  (oppure `install.ps1 -Uninstall`, con `-RemoveData` cancella anche la config)

### Dal sorgente

```bash
cargo run
cargo run -- --serve-only
cargo test
cargo build --release
```

## Collegare il telefono

1. Avvia TapDeck sul PC.
2. Apri Safari/Chrome e inquadra il **QR** (o digita `http://IP-del-PC:8787`).
3. **Fullscreen:**
   - **iPhone**: *Condividi* → *Aggiungi alla schermata Home*.
   - **Android**: *Installa app* / Aggiungi alla schermata Home; usa ⛶ per il pieno schermo.
4. Cambia pagina con swipe o coi puntini.
5. Premi un tasto: l'azione parte sul PC.

## Configurare i tasti

- **Pagine**: colonna a sinistra.
- **Griglia**: clicca una cella per aggiungere un tasto.
- **Icona**: catalogo, file PNG o URL.
- **Azioni** (tutte sul PC):
  - *Apre un programma* (percorso riconosciuto automaticamente)
  - *Apre un sito*
  - *Chiude un'app*
- **PIN** opzionale nella colonna di destra.

## Sicurezza e privacy

- La config resta sul tuo PC (`%APPDATA%\TapDeck\config.json`).
- Il telefono non riceve mai i percorsi `.exe`.
- Nessun account, nessuna telemetria.

## Limiti

- iPhone via USB da Windows: non possibile (limitazione Apple).
- Solo Windows sul lato PC; telefono con browser moderno (iOS 13+ / Android 9+).

## Contribuire

Leggi [CONTRIBUTING.md](CONTRIBUTING.md).

## Licenza

MIT — vedi [LICENSE](LICENSE).
