//
// CoreFlow Frontend - app.js
// Lógica para interagir com a API Rust e atualizar a interface.
//

document.addEventListener('DOMContentLoaded', () => {
    const loginScreen = document.getElementById('login-screen');
    const mainContent = document.getElementById('main-content');
    const loginForm = document.getElementById('login-form');
    const loginMessage = document.getElementById('login-message');
    const consoleLog = document.getElementById('console-log');
    const hardwareDetails = document.getElementById('hardware-details');
    const statusMessage = document.getElementById('status-message');
    const updateHardwareBtn = document.getElementById('update-hardware-btn');
    const actionButtons = document.querySelectorAll('.action-button');

    /**
     * Adiciona uma mensagem ao console de log.
     * @param {string} message - A mensagem a ser logada.
     * @param {string} type - Tipo de mensagem ('info', 'success', 'error').
     */
    function logToConsole(message, type = 'info') {
        const timestamp = new Date().toLocaleTimeString();
        let prefix = `[${timestamp}] [${type.toUpperCase()}] `;
        
        // Cor do console (usando códigos de escape ANSI simples, embora o browser não os interprete, é uma boa prática)
        let color = '';
        if (type === 'success') color = '#A4EA4F';
        else if (type === 'error') color = 'red';
        else color = '#0f0';

        const line = document.createElement('pre');
        line.style.color = color;
        line.textContent = prefix + message;

        consoleLog.appendChild(line);
        consoleLog.scrollTop = consoleLog.scrollHeight; // Scroll para o final
    }

    /**
     * Chama um endpoint da API e atualiza o console.
     * @param {string} endpoint - O caminho da API (ex: '/api/limpar_temp').
     * @param {string} actionName - O nome amigável da ação.
     */
    async function executeAction(endpoint, actionName) {
        logToConsole(`Iniciando ação: ${actionName}...`, 'info');
        
        try {
            const response = await fetch(endpoint, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            
            const result = await response.json();
            
            // O backend Rust agora retorna "SUCCESS" ou "ERROR"
            const type = result.status === 'SUCCESS' ? 'success' : 'error';
            logToConsole(`Ação ${actionName} - ${result.status}`, type);
            
            // O campo de detalhes agora é 'details' (em inglês)
            if (result.details) {
                logToConsole(`Detalhes:\n${result.details}`, 'info');
            } else {
                logToConsole(`Mensagem: ${result.message}`, 'info');
            }

        } catch (error) {
            logToConsole(`Falha na comunicação com a API para a ação ${actionName}: ${error.message}`, 'error');
        }
    }

    /**
     * Atualiza o painel de informações de Hardware.
     */
    async function updateHardwareInfo() {
        hardwareDetails.textContent = 'Carregando informações de hardware...';
        
        try {
            const response = await fetch('/api/hardware');
            const result = await response.json();

            // O backend Rust agora retorna a struct HardwareInfo diretamente
            if (response.ok) {
                hardwareDetails.textContent = result.message; // O campo 'message' contém o resumo formatado
                logToConsole('Informações de Hardware atualizadas.', 'success');
            } else {
                hardwareDetails.textContent = `Erro ao carregar hardware: ${result.message}`;
                logToConsole('Erro ao carregar hardware.', 'error');
            }
        } catch (error) {
            hardwareDetails.textContent = `Erro de conexão: ${error.message}`;
            logToConsole('Erro ao conectar com a API para informações de hardware.', 'error');
        }
    }

    /**
     * Verifica o status do servidor ao carregar a página.
     */
    async function checkServerStatus() {
        try {
            // Apenas verifica se o servidor está de pé com a rota /api/ping
            const response = await fetch('/api/ping');
            const result = await response.json();

            if (response.ok && result.status === 'SUCCESS') {
                statusMessage.textContent = `Servidor: ${result.message}`;
                statusMessage.style.color = '#00FFFF'; // Ciano
                logToConsole('Conexão com a API estabelecida.', 'success');
                // Não carrega hardware aqui, pois o usuário precisa logar primeiro
            } else {
                throw new Error(result.message || 'Erro desconhecido');
            }
        } catch (error) {
            statusMessage.textContent = 'Aguardando conexão com a API... FALHA.';
            statusMessage.style.color = '#ff0000';
            logToConsole(`Falha ao conectar: ${error.message}. Certifique-se de que o executável Rust está rodando.`, 'error');
        }
    }

    /**
     * Lógica de Login
     */
    async function handleLogin(e) {
        e.preventDefault();
        loginMessage.textContent = 'Tentando login...';
        
        const username = document.getElementById('username').value;
        const password = document.getElementById('password').value;

        try {
            const response = await fetch('/api/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ username, password })
            });

            const result = await response.json();

            if (response.ok && result.status === 'SUCCESS') {
                // Login bem-sucedido
                loginMessage.textContent = result.message;
                loginMessage.style.color = '#00FFFF';
                
                // Oculta a tela de login e mostra o conteúdo principal
                loginScreen.style.display = 'none';
                mainContent.style.display = 'block';
                
                // Carrega as informações de hardware após o login
                updateHardwareInfo();
                
            } else {
                // Login falhou (401 Unauthorized)
                loginMessage.textContent = result.message || 'Credenciais inválidas.';
                loginMessage.style.color = 'red';
            }

        } catch (error) {
            loginMessage.textContent = `Erro de comunicação: ${error.message}`;
            loginMessage.style.color = 'red';
        }
    }

    // --- Mapeamento de Ações ---
    const actionMap = {
        // Ações existentes
	        // As rotas do backend Rust não usam /admin/ no prefixo
	        'limpar_temp': { endpoint: '/api/limpar_temp', name: 'Limpeza de Temp' },
	        'listar_redes': { endpoint: '/api/listar_redes', name: 'Listar Adaptadores de Rede' },
	        'otimizar_disco': { endpoint: '/api/otimizar_disco', name: 'Otimizar Disco (C:)' },
	        'checar_integridade': { endpoint: '/api/checar_integridade', name: 'Checar Integridade SFC' },
	        'limpar_dns': { endpoint: '/api/limpar_dns', name: 'Limpar Cache DNS' },
	        'reiniciar_servicos': { endpoint: '/api/reiniciar_servicos', name: 'Reiniciar Serviços BITS/WUAUSERV' },
	        'limpar_logs': { endpoint: '/api/limpar_logs', name: 'Limpar Logs de Eventos' },

	        // Ações 8, 9, 10 e 11
	        'disk_health': { endpoint: '/api/disk_health', name: 'Saúde do Disco (S.M.A.R.T.)' },
	        'startup_apps': { endpoint: '/api/startup_apps', name: 'Listar Apps de Inicialização' },
	        'game_mode': { endpoint: '/api/game_mode', name: 'Ativar Game Mode' },
	        'work_mode': { endpoint: '/api/work_mode', name: 'Ativar Work Mode' },
    };


    // --- Event Listeners ---
    
    // 1. Botões de Ação
    actionButtons.forEach(button => {
        button.addEventListener('click', (e) => {
            const actionKey = e.currentTarget.getAttribute('data-action');
            if (actionMap[actionKey]) {
                const { endpoint, name } = actionMap[actionKey];
                executeAction(endpoint, name);
            }
        });
    });

    // 2. Botão de Atualizar Hardware
    updateHardwareBtn.addEventListener('click', updateHardwareInfo);
    
    // 3. Formulário de Login
    loginForm.addEventListener('submit', handleLogin);

    // 4. Botão de Download de Log
    downloadLogBtn.addEventListener('click', downloadLog);

    // Inicia a verificação de status do servidor
    checkServerStatus();
});

    /**
     * Função para baixar o arquivo de log.
     */
    async function downloadLog() {
        logToConsole('Solicitando download do log de execução...', 'info');
        try {
            const response = await fetch('/api/get_log');
            
            if (response.ok) {
                const logText = await response.text();
                
                // Cria um Blob (Binary Large Object) com o conteúdo do log
                const blob = new Blob([logText], { type: 'text/plain' });
                
                // Cria um link temporário para iniciar o download
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'coreflow_execution_log.txt';
                document.body.appendChild(a);
                a.click();
                document.body.removeChild(a);
                URL.revokeObjectURL(url);
                
                logToConsole('Download do log iniciado com sucesso.', 'success');
            } else {
                const errorText = await response.text();
                logToConsole(`Erro ao baixar o log: ${errorText}`, 'error');
            }
        } catch (error) {
            logToConsole(`Falha na comunicação para baixar o log: ${error.message}`, 'error');
        }
    }