# Chunithm New Tasoller chuniio.dll
Sega had some update in Chunithm New and made some feature of chuniio.dll cannot work in Chunithm New. This project is to make Tasoller custom firmware work again.

This is created by Rust but there are many `unsafe` is used. Memory leaks may appear and the stability still need to be tested.

## Setup Instruction
1. Place `usb-server.exe`, `chuniio_x86.dll`, `chuniio_x64.dll` into bin folder.
2. Copy 2 sets of `segatools.ini` to `segatools_32.ini` and `segatools_64.ini`
3. Add this to `segatools_32.ini`
    ```ini
    [chuniio]
    path=chuniio_x86.dll
    ```
4. Add this to `segatools_64.ini`
    ```ini
    [chuniio]
    path=chuniio_x64.dll
    ```
5. Modify your `start.bat` to this
    ```cmd
    @echo off
    cd /d %~dp0

    start usb-server.exe

    timeout 3

    copy /Y segatools_64.ini segatools.ini
    start inject_x64.exe -d -k chusanhook_x64.dll amdaemon.exe -f -c config_common.json config_server.json config_client.json config_sp.json config_cvt.json

    timeout 3

    copy /Y segatools_32.ini segatools.ini
    inject_x86.exe -d -k chusanhook_x86.dll chusanApp.exe

    taskkill /f /im amdaemon.exe > nul 2>&1

    echo.
    echo Game processes have terminated
    pause
    ```

## Usage Instruction
1. Connect Tasoller with Custom Firmware
2. Start `usb-server.exe` (Don't need to start it seperately if you already modified the `start.bat`)
3. Start the game (With 2 sets of `segatools.ini`. `chuniio_x86.dll` injected to chusanApp.exe and `chuniio_x64.dll` injected to `amdeamon.exe`)
---

## Compile Instruction
### For `chuniio_x86.dll`
`cargo build --target=i686-pc-windows-msvc --release`

### For `chuniio_x64.dll` and `usb-server.exe`
`cargo build --target=x86_64-pc-windows-msvc --release`

---

## Acknowledgements
The original source of tasoller-chuniio is reverse-engineered and released by [@akiroz](https://dev.s-ul.net/akiroz/chuniio-tasoller). 