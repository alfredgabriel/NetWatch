# 📡 NetWatch
### Monitor de Conexiones de Red Salientes en Tiempo Real para Windows
*Real-time outbound network connection monitor for Windows. Fast, driverless, and open-source.*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Tauri v2](https://img.shields.io/badge/Built%20with-Tauri%20v2-24C8D8?logo=tauri&logoColor=white)](https://tauri.app/)
[![Svelte v5](https://img.shields.io/badge/Svelte-v5-FF3E00?logo=svelte&logoColor=white)](https://svelte.dev/)
[![Rust Backend](https://img.shields.io/badge/Rust-Backend-black?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![UI: Brutalist](https://img.shields.io/badge/UI-Brutalist%20Monochrome-black)](https://github.com/alfredgabriel/netwatch)

---

## 🇪🇸 Español

### 1. Visión y Propósito

**NetWatch** es un monitor de sockets y tráfico de red para Windows de alto rendimiento. Identifica en tiempo real cada conexión TCP y UDP que sale de tu ordenador, resolviendo de forma asíncrona la IP remota a su dominio oficial y vinculando cada socket con el proceso, PID y ruta exacta del ejecutable.

**100% sin drivers**: No requiere Npcap, WinPcap ni controladores a nivel de kernel. Utiliza directamente las APIs nativas de Windows (`iphlpapi.dll`) y tareas asíncronas en Rust (`tokio`).

---

### 🚀 Características Principales

- ⚡ **Sin drivers ni privilegios de kernel**: Consulta eficiente de tablas TCP/UDP nativas sin alterar tu pila de red.
- 🪶 **Consumo mínimo**: Menos de 40 MB de RAM y menos del 1% de uso de CPU.
- 🌐 **DNS Inverso Asíncrono**: Resolución automática en segundo plano con caché lock-free (`DashMap`).
- 🔍 **Mapeo por Proceso**: Visualiza sockets en lista plana o agrupados por ejecutable.
- 📊 **Telemetría y Métricas en Vivo**: Velocidad de subida y bajada, contador de conexiones activas y estados.
- 🖤 **Estética Brutalista B&N**: Diseño monocromático de alto contraste, tipografía monoespaciada y respuesta instantánea.

---

### 🛠️ Desarrollo y Compilación

#### Requisitos
- [Node.js](https://nodejs.org/) v18+
- [Rust](https://www.rust-lang.org/tools/install) con toolchain MSVC de Windows

#### Ejecutar en desarrollo
```bash
npm install
npm run tauri dev
```

#### Compilar instalador standalone (.exe)
```bash
npm run tauri build
```
El ejecutable se genera en:
`src-tauri/target/release/bundle/`

---

## 🇬🇧 English

### 1. Overview & Purpose

**NetWatch** is a lightweight, driverless desktop network connection monitor for Windows. It captures all active outbound TCP and UDP sockets in real time, mapping remote IP addresses to human-readable hostnames and linking each socket to its owning process.

**Zero kernel drivers**: Operates entirely via native Windows `iphlpapi.dll` and tokio-driven async workers, eliminating the need for Npcap or special drivers.

---

### 2. Architecture

- **Backend**: Rust + Tauri v2 + Windows `GetExtendedTcpTable` / `GetExtendedUdpTable`.
- **Frontend**: Svelte 5 + Tailwind CSS + Brutalist Monochrome Design System.
- **Cache**: Concurrent lock-free reverse DNS cache via `DashMap`.

---

### 3. Build & Run

```bash
npm install
npm run tauri dev      # Development
npm run tauri build    # Standalone release (.exe)
```

---

## 📜 License

Distributed under the **MIT License**. See `LICENSE` for details.
