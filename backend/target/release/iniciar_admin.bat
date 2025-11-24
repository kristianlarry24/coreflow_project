@echo off
::
:: Script de Inicialização CoreFlow 2.0
:: Este script garante que o servidor seja executado com privilégios de Administrador
:: e abre o navegador automaticamente.
::
:: INSTRUÇÕES:
:: 1. Compile o projeto Rust (cargo build).
:: 2. Mova este arquivo (.bat) para a pasta onde está o executável:
::    backend\target\debug\
:: 3. Renomeie o executável, se necessário, para 'coreflow_project.exe'.
:: 4. Execute este arquivo 'iniciar_admin.bat'.
::

set EXECUTAVEL=coreflow_project.exe

:: Verifica se o script está rodando como Administrador
net session >nul 2>&1
if %errorLevel% == 0 (
    goto :run_server
) else (
    goto :elevate
)

:elevate
    echo Solicitando elevação de privilégios para o CoreFlow 2.0...
    :: Inicia o script novamente com privilégios elevados
    powershell -Command "Start-Process -FilePath '%~dpnx0' -Verb RunAs"
    exit /b

:run_server
    echo =======================================================
    echo Servidor CoreFlow 2.0 (Rust Native) Iniciado
    echo URL: http://127.0.0.1:8080
    echo Credenciais de Teste: admin / coreflow2024
    echo =======================================================
    
    REM CRÍTICO: Muda o diretório de trabalho para o local do script.
    REM Isso garante que o executável encontre a pasta 'static/' e o arquivo de log.
    cd /d "%~dp0"
    
    REM Inicia o servidor Rust em segundo plano
    start /B %EXECUTAVEL%

    REM Aguarda 3 segundos para garantir que o servidor subiu
    timeout /t 3 /nobreak >nul

    REM Abre o navegador na página de login
    start http://127.0.0.1:8080

    echo.
    echo Pressione qualquer tecla para fechar o servidor e o script...
    pause >nul
    
    REM Encerra o processo do servidor ao fechar o script
    taskkill /IM %EXECUTAVEL% /F >nul 2>&1
    exit
