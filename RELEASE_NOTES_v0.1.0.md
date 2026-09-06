# TapDeck v0.1.0

Prima release pubblica.

## Cos'è

TapDeck trasforma iPhone o Android in una **Stream Deck fatta in casa**: griglia di tasti con loghi ufficiali che controllano il PC Windows via Wi‑Fi.

## Cosa include questa release

- Editor sul PC (pagine, griglia, icone, azioni)
- Server + PWA per il telefono (verticale/orizzontale, swipe, schermo intero)
- Azioni: apre programma, apre sito, chiude app
- Riconoscimento automatico di ~90 programmi comuni
- Catalogo loghi ufficiali
- PIN opzionale
- Installazione come vera app Windows (`install.ps1`)
- Interfaccia italiano / inglese
- QR code per collegare il telefono in un attimo

## Installazione

1. Scarica `tapdeck-windows.zip` da questa release
2. Estrailo
3. Esegui:
   ```powershell
   powershell -ExecutionPolicy Bypass -File .\install.ps1
   ```
4. Avvia TapDeck dal Menu Start
5. Sul telefono apri l'indirizzo mostrato (o inquadra il QR)

**Requisiti:** Windows 10/11, telefono e PC sulla stessa rete Wi‑Fi.

## Note

- Al primo avvio Windows può chiedere il permesso firewall: scegli **rete privata**
- Su iPhone: *Condividi* → *Aggiungi alla schermata Home* per lo schermo intero
- Licenza MIT

Repository: https://github.com/xqm67/tapdeck
