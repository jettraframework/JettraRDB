use crate::storage::DatabaseRecordItem;

pub struct WebTemplates;

impl WebTemplates {
    pub fn login_page() -> String {
        r###"<!DOCTYPE html>
<html lang="es" data-bs-theme="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Iniciar Sesión - JettraRDB Management Console</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css">
    <style>
        :root {
            --jettra-primary: #f97316;
            --jettra-bg: #0b1120;
            --jettra-card: #1e293b;
            --jettra-border: #334155;
            --jettra-accent: #ea580c;
        }
        body {
            background-color: var(--jettra-bg);
            color: #f1f5f9;
            font-family: system-ui, -apple-system, sans-serif;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            background-image: radial-gradient(circle at 50% 20%, rgba(249, 115, 22, 0.08) 0%, transparent 60%);
        }
        .login-card {
            background-color: var(--jettra-card);
            border: 1px solid var(--jettra-border);
            border-radius: 14px;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.45);
            max-width: 440px;
            width: 100%;
        }
        .brand-logo {
            font-size: 2.75rem;
            line-height: 1;
        }
        .btn-jettra {
            background: linear-gradient(135deg, #f97316, #ea580c);
            border: none;
            color: #ffffff;
            font-weight: 600;
            padding: 0.75rem;
            transition: all 0.2s ease-in-out;
        }
        .btn-jettra:hover {
            background: linear-gradient(135deg, #fb923c, #f97316);
            color: #ffffff;
            box-shadow: 0 4px 12px rgba(249, 115, 22, 0.35);
        }
        .form-control:focus {
            background-color: #0f172a;
            border-color: var(--jettra-primary);
            box-shadow: 0 0 0 0.25rem rgba(249, 115, 22, 0.25);
            color: #f8fafc;
        }
        .form-control {
            background-color: #0f172a;
            border: 1px solid var(--jettra-border);
            color: #f8fafc;
        }
        .input-group-text {
            background-color: #0f172a;
            border: 1px solid var(--jettra-border);
            color: #94a3b8;
        }
        .badge-preset {
            cursor: pointer;
            transition: all 0.15s;
        }
        .badge-preset:hover {
            opacity: 0.85;
            transform: scale(1.02);
        }
    </style>
</head>
<body>
    <div class="container p-3">
        <div class="row justify-content-center">
            <div class="col-12 col-sm-10 col-md-8 col-lg-5">
                <!-- Branding Header -->
                <div class="text-center mb-4">
                    <div class="brand-logo mb-2">🦀</div>
                    <h2 class="fw-bold mb-1 text-white">Jettra<span style="color: var(--jettra-primary);">RDB</span></h2>
                    <p class="text-secondary small mb-0">Consola de Administración Web (JettraFlux)</p>
                    <span class="badge bg-warning text-dark mt-1" style="font-size: 0.7rem;">RUST v1.0 • Multi-Engine</span>
                </div>

                <!-- Main Login Card -->
                <div id="loginCard" class="card login-card p-4 p-md-5">
                    <h4 class="fw-semibold text-white mb-2"><i class="bi bi-box-arrow-in-right text-warning me-2"></i>Iniciar Sesión</h4>
                    <p class="text-secondary small mb-4">Ingrese sus credenciales de acceso al motor de datos.</p>

                    <!-- Alert Box -->
                    <div id="loginAlert" class="alert alert-danger d-none d-flex align-items-center py-2 px-3 small mb-3" role="alert">
                        <i class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2 fs-5"></i>
                        <div id="loginAlertMsg">Credenciales inválidas.</div>
                    </div>

                    <form id="loginForm" onsubmit="handleLogin(event)">
                        <div class="mb-3">
                            <label for="username" class="form-label text-secondary small fw-bold">USUARIO</label>
                            <div class="input-group">
                                <span class="input-group-text"><i class="bi bi-person-fill"></i></span>
                                <input type="text" id="username" class="form-control" placeholder="Ej. admin" required autocomplete="username" autofocus>
                            </div>
                        </div>

                        <div class="mb-4">
                            <label for="password" class="form-label text-secondary small fw-bold">CONTRASEÑA</label>
                            <div class="input-group">
                                <span class="input-group-text"><i class="bi bi-key-fill"></i></span>
                                <input type="password" id="password" class="form-control" placeholder="Contraseña" required autocomplete="current-password">
                                <button class="btn btn-outline-secondary" type="button" id="togglePasswordBtn" onclick="togglePassword()" title="Mostrar/ocultar contraseña">
                                    <i class="bi bi-eye" id="toggleIcon"></i>
                                </button>
                            </div>
                        </div>

                        <button type="submit" id="btnLogin" class="btn btn-jettra w-100 mb-3">
                            <i class="bi bi-box-arrow-in-right me-2"></i>Iniciar Sesión
                        </button>
                    </form>

                    <!-- Quick Preset Badges -->
                    <div class="border-top border-secondary pt-3 mt-2">
                        <div class="text-secondary small mb-2"><i class="bi bi-lightning-charge-fill text-warning me-1"></i>Credenciales del sistema:</div>
                        <div class="d-flex flex-wrap gap-2">
                            <span class="badge bg-dark border border-secondary text-light p-2 badge-preset" onclick="quickFill('admin', 'admin')" title="Haga clic para autocompletar">
                                <i class="bi bi-shield-lock text-info me-1"></i><strong>admin</strong> / admin
                            </span>
                            <span class="badge bg-dark border border-secondary text-light p-2 badge-preset" onclick="quickFill('super-user', 'superUserZ')" title="Haga clic para autocompletar">
                                <i class="bi bi-person-badge text-danger me-1"></i><strong>super-user</strong> / superUserZ
                            </span>
                        </div>
                    </div>
                </div>

                <!-- Password Change Card (Triggered if requires_password_change is true) -->
                <div id="changePwdCard" class="card login-card p-4 p-md-5 d-none">
                    <h4 class="fw-semibold text-white mb-2"><i class="bi bi-shield-exclamation text-warning me-2"></i>Actualización Requerida</h4>
                    <p class="text-secondary small mb-4">Esta cuenta requiere configurar una nueva contraseña antes de continuar.</p>

                    <div id="changePwdAlert" class="alert alert-danger d-none d-flex align-items-center py-2 px-3 small mb-3" role="alert">
                        <i class="bi bi-exclamation-circle-fill flex-shrink-0 me-2 fs-5"></i>
                        <div id="changePwdAlertMsg">Error al cambiar contraseña.</div>
                    </div>

                    <form id="changePwdForm" onsubmit="handleChangePassword(event)">
                        <input type="hidden" id="changePwdUsername">
                        <div class="mb-3">
                            <label class="form-label text-secondary small fw-bold">CONTRASEÑA ACTUAL</label>
                            <input type="password" id="changePwdOld" class="form-control" readonly>
                        </div>
                        <div class="mb-3">
                            <label for="changePwdNew" class="form-label text-secondary small fw-bold">NUEVA CONTRASEÑA</label>
                            <input type="password" id="changePwdNew" class="form-control" placeholder="Mínimo 4 caracteres" required minlength="4">
                        </div>
                        <div class="mb-4">
                            <label for="changePwdConfirm" class="form-label text-secondary small fw-bold">CONFIRMAR NUEVA CONTRASEÑA</label>
                            <input type="password" id="changePwdConfirm" class="form-control" placeholder="Repita la nueva contraseña" required minlength="4">
                        </div>
                        <button type="submit" id="btnChangePwd" class="btn btn-jettra w-100 mb-2">
                            <i class="bi bi-shield-check me-2"></i>Actualizar y Entrar
                        </button>
                    </form>
                </div>

                <div class="text-center mt-4 text-secondary small">
                    <span>JettraRDB &bull; Tokio Async &bull; Raft Consensus Engine</span>
                </div>
            </div>
        </div>
    </div>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
    <script>
        (async function checkExistingToken() {
            const token = localStorage.getItem("jettra_token");
            if (token) {
                try {
                    const res = await fetch("/api/auth/validate", {
                        headers: { "Authorization": "Bearer " + token }
                    });
                    if (res.ok) {
                        const data = await res.json();
                        if (data.valid) {
                            window.location.replace("/dashboard");
                        }
                    }
                } catch (e) {
                    console.error("Token verification error:", e);
                }
            }
        })();

        function quickFill(user, pass) {
            document.getElementById("username").value = user;
            document.getElementById("password").value = pass;
            document.getElementById("password").focus();
        }

        function togglePassword() {
            const pwd = document.getElementById("password");
            const icon = document.getElementById("toggleIcon");
            if (pwd.type === "password") {
                pwd.type = "text";
                icon.classList.remove("bi-eye");
                icon.classList.add("bi-eye-slash");
            } else {
                pwd.type = "password";
                icon.classList.remove("bi-eye-slash");
                icon.classList.add("bi-eye");
            }
        }

        async function handleLogin(e) {
            if (e) e.preventDefault();
            const alertBox = document.getElementById("loginAlert");
            const alertMsg = document.getElementById("loginAlertMsg");
            const btn = document.getElementById("btnLogin");
            const username = document.getElementById("username").value.trim();
            const password = document.getElementById("password").value;

            alertBox.classList.add("d-none");
            btn.disabled = true;
            btn.innerHTML = '<span class="spinner-border spinner-border-sm me-2"></span> Verificando...';

            try {
                const response = await fetch("/api/auth/login", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ username: username, password: password })
                });

                const data = await response.json();

                if (!response.ok) {
                    throw new Error(data.error || "Credenciales incorrectas o usuario no encontrado");
                }

                localStorage.setItem("jettra_token", data.token);
                localStorage.setItem("jettra_user", data.user);

                if (data.requires_password_change) {
                    document.getElementById("loginCard").classList.add("d-none");
                    document.getElementById("changePwdCard").classList.remove("d-none");
                    document.getElementById("changePwdUsername").value = data.user;
                    document.getElementById("changePwdOld").value = password;
                    document.getElementById("changePwdNew").focus();
                } else {
                    window.location.replace("/dashboard");
                }
            } catch (err) {
                alertMsg.textContent = err.message;
                alertBox.classList.remove("d-none");
            } finally {
                btn.disabled = false;
                btn.innerHTML = '<i class="bi bi-box-arrow-in-right me-2"></i> Iniciar Sesión';
            }
        }

        async function handleChangePassword(e) {
            if (e) e.preventDefault();
            const alertBox = document.getElementById("changePwdAlert");
            const alertMsg = document.getElementById("changePwdAlertMsg");
            const btn = document.getElementById("btnChangePwd");

            const username = document.getElementById("changePwdUsername").value.trim();
            const old_password = document.getElementById("changePwdOld").value;
            const new_password = document.getElementById("changePwdNew").value;
            const confirm_password = document.getElementById("changePwdConfirm").value;

            alertBox.classList.add("d-none");

            if (new_password !== confirm_password) {
                alertMsg.textContent = "Las nuevas contraseñas no coinciden.";
                alertBox.classList.remove("d-none");
                return;
            }
            if (new_password.length < 4) {
                alertMsg.textContent = "La nueva contraseña debe tener al menos 4 caracteres.";
                alertBox.classList.remove("d-none");
                return;
            }

            btn.disabled = true;
            btn.innerHTML = '<span class="spinner-border spinner-border-sm me-2"></span> Guardando...';

            try {
                const response = await fetch("/api/auth/change-password", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ username: username, old_password: old_password, new_password: new_password })
                });

                const data = await response.json();

                if (!response.ok) {
                    throw new Error(data.error || "Error al actualizar la contraseña");
                }

                window.location.replace("/dashboard");
            } catch (err) {
                alertMsg.textContent = err.message;
                alertBox.classList.remove("d-none");
            } finally {
                btn.disabled = false;
                btn.innerHTML = '<i class="bi bi-shield-check me-2"></i> Actualizar y Entrar';
            }
        }
    </script>
</body>
</html>"###.to_string()
    }

    pub fn layout(title: &str, active_nav: &str, content: &str) -> String {
        format!(r###"<!DOCTYPE html>
<html lang="en" data-bs-theme="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - JettraRDB Management Console</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css">
    <style>
        :root {{
            --jettra-primary: #f97316;
            --jettra-bg: #0f172a;
            --jettra-card: #1e293b;
            --jettra-border: #334155;
        }}
        body {{
            background-color: var(--jettra-bg);
            color: #f1f5f9;
            font-family: system-ui, -apple-system, sans-serif;
        }}
        .navbar-jettra {{
            background-color: #090d16;
            border-bottom: 1px solid var(--jettra-border);
        }}
        .sidebar {{
            min-height: calc(100vh - 56px);
            background-color: #0b1120;
            border-right: 1px solid var(--jettra-border);
        }}
        .nav-link {{
            color: #94a3b8;
            font-weight: 500;
            padding: 0.75rem 1rem;
            border-radius: 6px;
            margin: 2px 8px;
        }}
        .nav-link:hover, .nav-link.active {{
            color: #ffffff;
            background-color: #1e293b;
        }}
        .nav-link.active {{
            border-left: 3px solid var(--jettra-primary);
        }}
        .card-custom {{
            background-color: var(--jettra-card);
            border: 1px solid var(--jettra-border);
            border-radius: 10px;
        }}
        .badge-engine {{
            background: linear-gradient(135deg, #f97316, #ea580c);
            font-weight: 600;
        }}
        .code-box {{
            background-color: #090d16;
            border: 1px solid var(--jettra-border);
            border-radius: 6px;
            padding: 12px;
            font-family: monospace;
            color: #38bdf8;
            overflow-x: auto;
        }}
    </style>
</head>
<body>
    <nav class="navbar navbar-expand-lg navbar-jettra sticky-top px-3">
        <a class="navbar-brand d-flex align-items-center gap-2" href="/dashboard">
            <span class="fs-4">🦀</span>
            <span class="fw-bold text-white">Jettra<span style="color: var(--jettra-primary);">RDB</span></span>
            <span class="badge bg-warning text-dark ms-2" style="font-size: 0.7rem;">RUST v1.0</span>
        </a>
        <div class="ms-auto d-flex align-items-center gap-3">
            <span class="text-secondary small d-none d-md-inline"><i class="bi bi-hdd-network text-success"></i> Cluster: Online</span>
            <a href="/swagger-ui" class="btn btn-sm btn-outline-warning"><i class="bi bi-file-code"></i> OpenAPI / Swagger</a>
            <div id="navUserSection" class="d-flex align-items-center gap-2 border-start border-secondary ps-3">
                <span class="badge bg-dark border border-secondary text-light px-2 py-1">
                    <i class="bi bi-person-circle text-warning me-1"></i><span id="navUserText">admin</span>
                </span>
                <button onclick="logout()" class="btn btn-sm btn-outline-danger" title="Cerrar sesión">
                    <i class="bi bi-box-arrow-right me-1"></i>Salir
                </button>
            </div>
        </div>
    </nav>

    <div class="container-fluid">
        <div class="row">
            <nav class="col-md-3 col-lg-2 d-md-block sidebar py-3">
                <div class="position-sticky">
                    <ul class="nav flex-column">
                        <li class="nav-item">
                            <a class="nav-link {nav_dashboard}" href="/dashboard">
                                <i class="bi bi-speedometer2 me-2"></i> Dashboard
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link {nav_databases}" href="/databases">
                                <i class="bi bi-database me-2"></i> Databases
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link {nav_explorer}" href="/explorer">
                                <i class="bi bi-table me-2"></i> Data Explorer
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link {nav_engines}" href="/engines">
                                <i class="bi bi-cpu me-2"></i> 9 Multi-Model Engines
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link {nav_users}" href="/users">
                                <i class="bi bi-people me-2"></i> Users & RBAC
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link {nav_components}" href="/components">
                                <i class="bi bi-diagram-3 me-2"></i> Cluster Topology
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link {nav_information}" href="/information">
                                <i class="bi bi-info-circle me-2"></i> Architecture Info
                            </a>
                        </li>
                        <li class="nav-item mt-3 pt-3 border-top border-secondary">
                            <a class="nav-link text-warning" href="/swagger-ui">
                                <i class="bi bi-terminal me-2"></i> Swagger API Explorer
                            </a>
                        </li>
                    </ul>
                </div>
            </nav>

            <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4 py-4">
                {content}
            </main>
        </div>
    </div>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
    <script>
        (function() {{
            const token = localStorage.getItem("jettra_token");
            const user = localStorage.getItem("jettra_user");
            if (!token) {{
                window.location.replace("/login");
                return;
            }}
            const userEl = document.getElementById("navUserText");
            if (userEl && user) {{
                userEl.textContent = user;
            }}
        }})();

        function logout() {{
            localStorage.removeItem("jettra_token");
            localStorage.removeItem("jettra_user");
            window.location.replace("/login");
        }}
    </script>
</body>
</html>"###,
            title = title,
            nav_dashboard = if active_nav == "dashboard" { "active" } else { "" },
            nav_databases = if active_nav == "databases" { "active" } else { "" },
            nav_explorer = if active_nav == "explorer" { "active" } else { "" },
            nav_engines = if active_nav == "engines" { "active" } else { "" },
            nav_users = if active_nav == "users" { "active" } else { "" },
            nav_components = if active_nav == "components" { "active" } else { "" },
            nav_information = if active_nav == "information" { "active" } else { "" },
            content = content
        )
    }

    pub fn dashboard_page(databases_count: usize, engines_count: usize, node_id: &str) -> String {
        let content = format!(r#"
        <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-2 pb-3 mb-4 border-bottom border-secondary">
            <h1 class="h2"><i class="bi bi-speedometer2 text-warning me-2"></i>System Dashboard</h1>
            <div class="btn-toolbar mb-2 mb-md-0">
                <span class="badge bg-success p-2"><i class="bi bi-check-circle-fill me-1"></i> Autonomous Node: {node_id}</span>
            </div>
        </div>

        <div class="row g-3 mb-4">
            <div class="col-sm-6 col-xl-3">
                <div class="card card-custom p-3">
                    <div class="d-flex align-items-center">
                        <div class="bg-primary bg-opacity-10 text-primary p-3 rounded-3 me-3">
                            <i class="bi bi-database fs-3"></i>
                        </div>
                        <div>
                            <div class="text-secondary small">Databases</div>
                            <div class="fs-4 fw-bold text-white">{databases_count}</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-sm-6 col-xl-3">
                <div class="card card-custom p-3">
                    <div class="d-flex align-items-center">
                        <div class="bg-warning bg-opacity-10 text-warning p-3 rounded-3 me-3">
                            <i class="bi bi-cpu fs-3"></i>
                        </div>
                        <div>
                            <div class="text-secondary small">Active Engines</div>
                            <div class="fs-4 fw-bold text-white">{engines_count} Multi-Model</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-sm-6 col-xl-3">
                <div class="card card-custom p-3">
                    <div class="d-flex align-items-center">
                        <div class="bg-success bg-opacity-10 text-success p-3 rounded-3 me-3">
                            <i class="bi bi-lightning-charge fs-3"></i>
                        </div>
                        <div>
                            <div class="text-secondary small">Runtime Engine</div>
                            <div class="fs-4 fw-bold text-white">Rust (Tokio async)</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-sm-6 col-xl-3">
                <div class="card card-custom p-3">
                    <div class="d-flex align-items-center">
                        <div class="bg-info bg-opacity-10 text-info p-3 rounded-3 me-3">
                            <i class="bi bi-shield-lock fs-3"></i>
                        </div>
                        <div>
                            <div class="text-secondary small">GC Latency</div>
                            <div class="fs-4 fw-bold text-white">0 ms (Zero GC)</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="row g-4">
            <div class="col-lg-8">
                <div class="card card-custom p-4">
                    <h5 class="card-title text-white mb-3"><i class="bi bi-layers text-warning me-2"></i>Unified 9 Multi-Model Architecture</h5>
                    <p class="text-secondary">JettraRDB unifies 9 distinct database operational models over a single resilient LSM-Tree + B-Tree hybrid storage core written natively in Rust.</p>
                    <div class="row g-2">
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">1</span> <strong>DOCUMENT</strong><br><small class="text-secondary">NoSQL JSON / BSON</small></div></div>
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">2</span> <strong>VECTOR</strong><br><small class="text-secondary">AI Embeddings & ANN</small></div></div>
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">3</span> <strong>GRAPH</strong><br><small class="text-secondary">LPG Nodes & Traversal</small></div></div>
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">4</span> <strong>TIMESERIES</strong><br><small class="text-secondary">IoT Metrics & Telemetry</small></div></div>
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">5</span> <strong>COLUMN</strong><br><small class="text-secondary">OLAP Column Families</small></div></div>
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">6</span> <strong>KEYVALUE</strong><br><small class="text-secondary">Atomic MemTable Cache</small></div></div>
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">7</span> <strong>GEOSPATIAL</strong><br><small class="text-secondary">2D GIS & Haversine</small></div></div>
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">8</span> <strong>OBJECT</strong><br><small class="text-secondary">Binary BLOBs & Streams</small></div></div>
                        <div class="col-md-4"><div class="p-2 border border-secondary rounded bg-dark"><span class="badge badge-engine me-1">9</span> <strong>RECORDS</strong><br><small class="text-secondary">Schema & Projections</small></div></div>
                    </div>
                </div>
            </div>
            <div class="col-lg-4">
                <div class="card card-custom p-4">
                    <h5 class="card-title text-white mb-3"><i class="bi bi-hdd-stack text-warning me-2"></i>Storage Engine Internals</h5>
                    <ul class="list-unstyled text-secondary small">
                        <li class="mb-2"><strong class="text-light">MemTable:</strong> Concurrent Lock-free BTreeMap</li>
                        <li class="mb-2"><strong class="text-light">WAL:</strong> Binary append-only <code>wal.jettra</code></li>
                        <li class="mb-2"><strong class="text-light">SSTable:</strong> Block file <code>data_0.jettra</code></li>
                        <li class="mb-2"><strong class="text-light">Isolation:</strong> Dedicated per-database partition</li>
                        <li class="mb-2"><strong class="text-light">Versioning:</strong> Multi-Version Concurrency (MVCC)</li>
                        <li class="mb-2"><strong class="text-light">Raft Consensus:</strong> Distributed peer replication</li>
                    </ul>
                    <a href="/engines" class="btn btn-warning w-100 mt-2"><i class="bi bi-play-circle me-1"></i> Explore Engines Live</a>
                </div>
            </div>
        </div>
        "#, databases_count = databases_count, engines_count = engines_count, node_id = node_id);

        Self::layout("Dashboard", "dashboard", &content)
    }

    pub fn databases_page(databases: &[String]) -> String {
        let mut rows = String::new();
        for db in databases {
            let is_system = db.eq_ignore_ascii_case("_system");
            let drop_btn = if is_system {
                r#"<button class="btn btn-sm btn-outline-secondary" disabled title="System database cannot be dropped"><i class="bi bi-trash"></i> Drop</button>"#.to_string()
            } else {
                format!(r#"<button class="btn btn-sm btn-outline-danger" onclick="dropDb('{}')"><i class="bi bi-trash"></i> Drop</button>"#, db)
            };

            let rename_btn = if is_system {
                r#"<button class="btn btn-sm btn-outline-secondary" disabled title="System database cannot be renamed"><i class="bi bi-pencil-square"></i> Renombrar</button>"#.to_string()
            } else {
                format!(r#"<button class="btn btn-sm btn-outline-warning me-1" onclick="openRenameModal('{}')"><i class="bi bi-pencil-square"></i> Renombrar</button>"#, db)
            };

            rows.push_str(&format!(
                r#"<tr>
                    <td class="fw-bold text-white">
                        <i class="bi bi-database me-2 text-warning"></i>
                        <a href="/explorer?db={}" class="text-white text-decoration-none">{}</a>
                        {}
                    </td>
                    <td><span class="badge bg-success">ONLINE</span></td>
                    <td><code>data/databases/{}/</code></td>
                    <td>
                        <a href="/explorer?db={}" class="btn btn-sm btn-outline-info me-1"><i class="bi bi-table"></i> Explorar</a>
                        {}
                        {}
                    </td>
                </tr>"#,
                db,
                db,
                if is_system { r#"<span class="badge bg-secondary ms-2">SYSTEM</span>"# } else { "" },
                db,
                db,
                rename_btn,
                drop_btn
            ));
        }

        if databases.is_empty() {
            rows = r#"<tr><td colspan="4" class="text-center text-secondary py-4">No hay bases de datos creadas. Haga clic en 'Nueva Base de Datos' para comenzar.</td></tr>"#.to_string();
        }

        let content = format!(r###"
        <div class="d-flex justify-content-between align-items-center pt-2 pb-3 mb-4 border-bottom border-secondary">
            <div>
                <h1 class="h2 mb-1"><i class="bi bi-database text-warning me-2"></i>Databases Management</h1>
                <p class="text-secondary small mb-0">Administración de particiones multi-modelo aisladas en disco.</p>
            </div>
            <button class="btn btn-warning" data-bs-toggle="modal" data-bs-target="#createDbModal">
                <i class="bi bi-plus-circle me-1"></i> Nueva Base de Datos
            </button>
        </div>

        <div class="card card-custom p-4">
            <h5 class="card-title text-white mb-3">Particiones Aisladas de Base de Datos</h5>
            <div class="table-responsive">
                <table class="table table-dark table-hover align-middle">
                    <thead>
                        <tr>
                            <th>Nombre de Base de Datos</th>
                            <th>Estado</th>
                            <th>Ruta en Disco</th>
                            <th>Acciones</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>
        </div>

        <!-- Modal Crear DB -->
        <div class="modal fade" id="createDbModal" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog">
                <div class="modal-content bg-dark text-light border-secondary">
                    <div class="modal-header border-secondary">
                        <h5 class="modal-title"><i class="bi bi-plus-circle text-warning me-2"></i>Nueva Base de Datos</h5>
                        <button type="button" class="btn-close btn-close-white" data-bs-dismiss="modal"></button>
                    </div>
                    <div class="modal-body">
                        <div class="mb-3">
                            <label class="form-label text-secondary">Nombre de la Base de Datos</label>
                            <input type="text" id="newDbName" class="form-control bg-secondary bg-opacity-25 text-white border-secondary" placeholder="ej: ecommerce, analytics, iot_telemetry">
                            <div class="form-text text-secondary">Solo caracteres alfanuméricos, guiones (-) y guiones bajos (_).</div>
                        </div>
                        <div id="createDbError" class="alert alert-danger d-none py-2"></div>
                    </div>
                    <div class="modal-footer border-secondary">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancelar</button>
                        <button type="button" class="btn btn-warning" onclick="submitCreateDb()"><i class="bi bi-check-lg me-1"></i> Crear Base de Datos</button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Modal Renombrar DB -->
        <div class="modal fade" id="renameDbModal" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog">
                <div class="modal-content bg-dark text-light border-secondary">
                    <div class="modal-header border-secondary">
                        <h5 class="modal-title"><i class="bi bi-pencil-square text-warning me-2"></i>Renombrar Base de Datos</h5>
                        <button type="button" class="btn-close btn-close-white" data-bs-dismiss="modal"></button>
                    </div>
                    <div class="modal-body">
                        <input type="hidden" id="renameOldName">
                        <div class="mb-3">
                            <label class="form-label text-secondary">Nombre Actual</label>
                            <input type="text" id="renameCurrentNameDisplay" class="form-control bg-secondary bg-opacity-10 text-secondary border-secondary" readonly>
                        </div>
                        <div class="mb-3">
                            <label class="form-label text-secondary">Nuevo Nombre</label>
                            <input type="text" id="renameNewName" class="form-control bg-secondary bg-opacity-25 text-white border-secondary" placeholder="ej: nuevo_nombre">
                        </div>
                        <div id="renameDbError" class="alert alert-danger d-none py-2"></div>
                    </div>
                    <div class="modal-footer border-secondary">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancelar</button>
                        <button type="button" class="btn btn-warning" onclick="submitRenameDb()"><i class="bi bi-check-lg me-1"></i> Guardar Cambios</button>
                    </div>
                </div>
            </div>
        </div>

        <script>
            async function dropDb(name) {{
                if (confirm("¿Está seguro de que desea eliminar la base de datos '" + name + "'? Esta acción eliminará permanentemente todos sus archivos.")) {{
                    const res = await fetch("/api/databases/" + encodeURIComponent(name), {{ method: "DELETE" }});
                    if (res.ok) window.location.reload();
                    else alert("Error al eliminar la base de datos");
                }}
            }}

            async function submitCreateDb() {{
                const name = document.getElementById("newDbName").value.trim();
                const errDiv = document.getElementById("createDbError");
                errDiv.classList.add("d-none");

                if (!name) {{
                    errDiv.innerText = "Por favor especifique un nombre para la base de datos";
                    errDiv.classList.remove("d-none");
                    return;
                }}

                try {{
                    const res = await fetch("/api/databases", {{
                        method: "POST",
                        headers: {{ "Content-Type": "application/json" }},
                        body: JSON.stringify({{ name: name }})
                    }});
                    const data = await res.json();
                    if (res.ok) {{
                        window.location.reload();
                    }} else {{
                        errDiv.innerText = data.error || "Error al crear la base de datos";
                        errDiv.classList.remove("d-none");
                    }}
                }} catch (e) {{
                    errDiv.innerText = "Error de conexión: " + e.message;
                    errDiv.classList.remove("d-none");
                }}
            }}

            function openRenameModal(oldName) {{
                document.getElementById("renameOldName").value = oldName;
                document.getElementById("renameCurrentNameDisplay").value = oldName;
                document.getElementById("renameNewName").value = oldName + "_new";
                document.getElementById("renameDbError").classList.add("d-none");
                const modal = new bootstrap.Modal(document.getElementById("renameDbModal"));
                modal.show();
            }}

            async function submitRenameDb() {{
                const oldName = document.getElementById("renameOldName").value.trim();
                const newName = document.getElementById("renameNewName").value.trim();
                const errDiv = document.getElementById("renameDbError");
                errDiv.classList.add("d-none");

                if (!newName) {{
                    errDiv.innerText = "El nuevo nombre no puede estar vacío";
                    errDiv.classList.remove("d-none");
                    return;
                }}

                try {{
                    const res = await fetch("/api/databases/" + encodeURIComponent(oldName) + "/rename", {{
                        method: "PUT",
                        headers: {{ "Content-Type": "application/json" }},
                        body: JSON.stringify({{ new_name: newName }})
                    }});
                    const data = await res.json();
                    if (res.ok) {{
                        window.location.reload();
                    }} else {{
                        errDiv.innerText = data.error || "Error al renombrar la base de datos";
                        errDiv.classList.remove("d-none");
                    }}
                }} catch (e) {{
                    errDiv.innerText = "Error de conexión: " + e.message;
                    errDiv.classList.remove("d-none");
                }}
            }}
        </script>
        "###, rows);

        Self::layout("Databases", "databases", &content)
    }

    pub fn explorer_page(databases: &[String], active_db: &str, records: &[DatabaseRecordItem]) -> String {
        let mut db_options = String::new();
        for db in databases {
            let selected = if db == active_db { "selected" } else { "" };
            db_options.push_str(&format!(
                r#"<option value="{}" {}>{}</option>"#,
                db, selected, db
            ));
        }

        let records_json = serde_json::to_string(records).unwrap_or_else(|_| "[]".to_string());

        let content = format!(r###"
        <style>
            .tree-node {{
                list-style-type: none;
                margin: 0;
                padding: 4px 0;
            }}
            .tree-toggler {{
                cursor: pointer;
                user-select: none;
            }}
            .tree-toggler::before {{
                content: "▶ ";
                color: #f97316;
                display: inline-block;
                margin-right: 4px;
                font-size: 0.75rem;
                transition: transform 0.15s ease-in-out;
            }}
            .tree-toggler.open::before {{
                transform: rotate(90deg);
            }}
            .tree-children {{
                display: none;
                padding-left: 20px;
                border-left: 1px dashed #334155;
                margin-left: 8px;
            }}
            .tree-children.open {{
                display: block;
            }}
            .pill-filter.active {{
                background-color: var(--jettra-primary) !important;
                color: #fff !important;
            }}
            .payload-preview {{
                max-width: 320px;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
                font-family: monospace;
            }}
        </style>

        <div class="d-flex justify-content-between align-items-center flex-wrap gap-2 pt-2 pb-3 mb-4 border-bottom border-secondary">
            <div class="d-flex align-items-center gap-3 flex-wrap">
                <h1 class="h2 mb-0"><i class="bi bi-table text-warning me-2"></i>Data Explorer</h1>
                <div class="d-flex align-items-center gap-2 bg-dark p-1 px-2 rounded border border-secondary">
                    <span class="text-secondary small"><i class="bi bi-database text-warning"></i> BD:</span>
                    <select class="form-select form-select-sm bg-dark text-white border-0 py-0" style="width: auto; font-weight: 600;" onchange="changeDatabase(this.value)">
                        {db_options}
                    </select>
                </div>
                <span class="badge bg-secondary" id="recordCountBadge">{total_records} registros</span>
            </div>
            <div class="d-flex align-items-center gap-2">
                <div class="btn-group" role="group">
                    <button id="btnViewTable" class="btn btn-sm btn-outline-warning active" onclick="switchView('table')">
                        <i class="bi bi-table me-1"></i> Vista Tabla
                    </button>
                    <button id="btnViewTree" class="btn btn-sm btn-outline-warning" onclick="switchView('tree')">
                        <i class="bi bi-diagram-3 me-1"></i> Vista Árbol (Tree)
                    </button>
                </div>
                <button class="btn btn-sm btn-warning" onclick="openCreateModal()">
                    <i class="bi bi-plus-circle me-1"></i> Nuevo Registro
                </button>
            </div>
        </div>

        <!-- Filter & Search Toolbar -->
        <div class="card card-custom p-3 mb-4">
            <div class="row g-2 align-items-center">
                <div class="col-md-5">
                    <div class="input-group input-group-sm">
                        <span class="input-group-text bg-dark text-secondary border-secondary"><i class="bi bi-search"></i></span>
                        <input type="text" id="filterInput" class="form-control bg-dark text-white border-secondary" placeholder="Buscar por clave, namespace, ID o datos..." oninput="applyFilters()">
                        <button class="btn btn-outline-secondary" type="button" onclick="clearFilter()"><i class="bi bi-x"></i></button>
                    </div>
                </div>
                <div class="col-md-7">
                    <div class="d-flex flex-wrap gap-1 align-items-center" id="engineFilterPills">
                        <span class="badge bg-dark border border-secondary text-secondary me-1">Filtro Motor:</span>
                        <button class="btn btn-xs btn-outline-secondary pill-filter active py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('ALL')">TODOS</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('DOCUMENT')">DOCUMENT</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('VECTOR')">VECTOR</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('GRAPH')">GRAPH</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('TIMESERIES')">TIMESERIES</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('COLUMN')">COLUMN</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('KEYVALUE')">KEYVALUE</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('GEOSPATIAL')">GEOSPATIAL</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('OBJECT')">OBJECT</button>
                        <button class="btn btn-xs btn-outline-secondary pill-filter py-0 px-2" style="font-size: 0.75rem;" onclick="filterByEngine('RECORDS')">RECORDS</button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Vista 1: Tabla -->
        <div id="tableViewSection" class="card card-custom p-4">
            <div class="table-responsive">
                <table class="table table-dark table-hover align-middle mb-0" id="recordsTable">
                    <thead>
                        <tr>
                            <th style="width: 130px;">Motor</th>
                            <th>Clave (Key)</th>
                            <th>Namespace</th>
                            <th>ID</th>
                            <th>Contenido (Payload)</th>
                            <th style="width: 100px;">Versiones</th>
                            <th style="width: 220px;">Acciones</th>
                        </tr>
                    </thead>
                    <tbody id="tableBody">
                        <!-- Populated dynamically via JS -->
                    </tbody>
                </table>
            </div>
            <div id="tableEmptyMessage" class="text-center text-secondary py-4 d-none">
                <i class="bi bi-inbox fs-2 d-block mb-2"></i>
                No se encontraron registros en la base de datos activa. Utilice el botón <strong>Nuevo Registro</strong> para agregar uno.
            </div>
        </div>

        <!-- Vista 2: Árbol (Tree) -->
        <div id="treeViewSection" class="card card-custom p-4 d-none">
            <div class="d-flex justify-content-between align-items-center mb-3">
                <h5 class="card-title text-white mb-0"><i class="bi bi-diagram-3 text-warning me-2"></i>Estructura Jerárquica de Registros</h5>
                <div class="btn-group btn-group-sm">
                    <button class="btn btn-outline-secondary" onclick="expandAllTree()"><i class="bi bi-arrows-expand me-1"></i> Expandir Todo</button>
                    <button class="btn btn-outline-secondary" onclick="collapseAllTree()"><i class="bi bi-arrows-collapse me-1"></i> Colapsar Todo</button>
                </div>
            </div>
            <div class="bg-dark p-3 rounded border border-secondary" style="min-height: 250px; font-family: monospace;">
                <ul class="list-unstyled mb-0" id="treeRoot">
                    <!-- Populated dynamically via JS -->
                </ul>
            </div>
        </div>

        <!-- Modal Ver Registro -->
        <div class="modal fade" id="viewRecordModal" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content bg-dark text-light border-secondary">
                    <div class="modal-header border-secondary">
                        <h5 class="modal-title d-flex align-items-center gap-2">
                            <span id="viewModalEngineBadge"></span>
                            <span>Registro: <code id="viewModalId" class="text-warning"></code></span>
                        </h5>
                        <button type="button" class="btn-close btn-close-white" data-bs-dismiss="modal"></button>
                    </div>
                    <div class="modal-body">
                        <div class="row g-2 mb-3">
                            <div class="col-sm-4"><small class="text-secondary">Namespace:</small> <div id="viewModalNs" class="fw-bold"></div></div>
                            <div class="col-sm-4"><small class="text-secondary">Versiones (MVCC):</small> <div id="viewModalVersions" class="fw-bold"></div></div>
                            <div class="col-sm-4"><small class="text-secondary">Clave Primaria:</small> <div id="viewModalKey" class="small text-truncate"></div></div>
                        </div>
                        <div class="d-flex justify-content-between align-items-center mb-1">
                            <span class="text-secondary small">Payload JSON:</span>
                            <button class="btn btn-xs btn-outline-info py-0 px-2" onclick="copyViewJson()"><i class="bi bi-clipboard"></i> Copiar</button>
                        </div>
                        <pre id="viewModalPayload" class="code-box text-light" style="max-height: 400px; overflow: auto;"></pre>
                    </div>
                    <div class="modal-footer border-secondary">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cerrar</button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Modal Crear Registro (Dynamic Multi-Model) -->
        <div class="modal fade" id="createRecordModal" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content bg-dark text-light border-secondary">
                    <div class="modal-header border-secondary">
                        <h5 class="modal-title"><i class="bi bi-plus-circle text-warning me-2"></i>Crear Nuevo Registro Multi-Modelo</h5>
                        <button type="button" class="btn-close btn-close-white" data-bs-dismiss="modal"></button>
                    </div>
                    <div class="modal-body">
                        <div class="row g-3 mb-3">
                            <div class="col-md-4">
                                <label class="form-label text-secondary">Motor (Engine)</label>
                                <select id="createModalModel" class="form-select bg-dark text-white border-secondary" onchange="onModelSelectChange()">
                                    <option value="DOCUMENT">DOCUMENT (NoSQL)</option>
                                    <option value="VECTOR">VECTOR (AI Embeddings)</option>
                                    <option value="GRAPH">GRAPH (LPG Nodes)</option>
                                    <option value="TIMESERIES">TIMESERIES (IoT / Metrics)</option>
                                    <option value="COLUMN">COLUMN (Column Family)</option>
                                    <option value="KEYVALUE">KEYVALUE (MemTable Cache)</option>
                                    <option value="GEOSPATIAL">GEOSPATIAL (GIS 2D)</option>
                                    <option value="OBJECT">OBJECT (OOP States)</option>
                                    <option value="RECORDS">RECORDS (Schema & Projections)</option>
                                </select>
                            </div>
                            <div class="col-md-4">
                                <label class="form-label text-secondary">Namespace / Colección</label>
                                <input type="text" id="createModalNs" class="form-control bg-dark text-white border-secondary" value="{active_db}">
                            </div>
                            <div class="col-md-4">
                                <label class="form-label text-secondary">ID del Registro</label>
                                <input type="text" id="createModalId" class="form-control bg-dark text-white border-secondary" placeholder="ej: doc_01">
                            </div>
                        </div>
                        <div class="mb-3">
                            <div class="d-flex justify-content-between align-items-center mb-1">
                                <label class="form-label text-secondary mb-0">Contenido del Registro (JSON)</label>
                                <button type="button" class="btn btn-xs btn-outline-warning py-0 px-2" onclick="resetModelTemplate()"><i class="bi bi-arrow-counterclockwise"></i> Restaurar Plantilla</button>
                            </div>
                            <textarea id="createModalPayload" class="form-control bg-dark text-info border-secondary font-monospace" rows="10"></textarea>
                        </div>
                        <div id="createRecordError" class="alert alert-danger d-none py-2"></div>
                    </div>
                    <div class="modal-footer border-secondary">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancelar</button>
                        <button type="button" class="btn btn-warning" onclick="submitCreateRecord()"><i class="bi bi-check-lg me-1"></i> Guardar Registro</button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Modal Editar Registro -->
        <div class="modal fade" id="editRecordModal" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content bg-dark text-light border-secondary">
                    <div class="modal-header border-secondary">
                        <h5 class="modal-title"><i class="bi bi-pencil-square text-warning me-2"></i>Editar Registro</h5>
                        <button type="button" class="btn-close btn-close-white" data-bs-dismiss="modal"></button>
                    </div>
                    <div class="modal-body">
                        <div class="row g-3 mb-3">
                            <div class="col-md-4">
                                <label class="form-label text-secondary">Motor</label>
                                <input type="text" id="editModalModel" class="form-control bg-secondary bg-opacity-10 text-secondary border-secondary" readonly>
                            </div>
                            <div class="col-md-4">
                                <label class="form-label text-secondary">Namespace</label>
                                <input type="text" id="editModalNs" class="form-control bg-secondary bg-opacity-10 text-secondary border-secondary" readonly>
                            </div>
                            <div class="col-md-4">
                                <label class="form-label text-secondary">ID</label>
                                <input type="text" id="editModalId" class="form-control bg-secondary bg-opacity-10 text-secondary border-secondary" readonly>
                            </div>
                        </div>
                        <div class="mb-3">
                            <label class="form-label text-secondary">Contenido (JSON)</label>
                            <textarea id="editModalPayload" class="form-control bg-dark text-info border-secondary font-monospace" rows="10"></textarea>
                        </div>
                        <div id="editRecordError" class="alert alert-danger d-none py-2"></div>
                    </div>
                    <div class="modal-footer border-secondary">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancelar</button>
                        <button type="button" class="btn btn-warning" onclick="submitEditRecord()"><i class="bi bi-check-lg me-1"></i> Guardar Cambios</button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Modal Historial MVCC -->
        <div class="modal fade" id="historyRecordModal" tabindex="-1" aria-hidden="true">
            <div class="modal-dialog modal-lg">
                <div class="modal-content bg-dark text-light border-secondary">
                    <div class="modal-header border-secondary">
                        <h5 class="modal-title"><i class="bi bi-clock-history text-warning me-2"></i>Historial de Versiones MVCC</h5>
                        <button type="button" class="btn-close btn-close-white" data-bs-dismiss="modal"></button>
                    </div>
                    <div class="modal-body">
                        <div class="mb-3">
                            <small class="text-secondary">Clave del Registro:</small> <code id="historyModalKey" class="text-warning"></code>
                        </div>
                        <div class="table-responsive">
                            <table class="table table-dark table-hover align-middle">
                                <thead>
                                    <tr>
                                        <th>Versión</th>
                                        <th>Fecha / Hora</th>
                                        <th>Contenido</th>
                                        <th>Estado</th>
                                        <th>Acción</th>
                                    </tr>
                                </thead>
                                <tbody id="historyTableBody">
                                    <!-- Populated dynamically via JS -->
                                </tbody>
                            </table>
                        </div>
                    </div>
                    <div class="modal-footer border-secondary">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cerrar</button>
                    </div>
                </div>
            </div>
        </div>

        <script>
            const ALL_RECORDS = {records_json};
            const ACTIVE_DB = "{active_db}";
            let currentFilterEngine = "ALL";
            let currentFilterText = "";
            let currentView = "table";

            const ENGINE_TEMPLATES = {{
                DOCUMENT: JSON.stringify({{
                    name: "Documento de Ejemplo",
                    status: "activo",
                    tags: ["rust", "jettra", "document"],
                    detalles: {{
                        creado_por: "admin",
                        version: 1
                    }}
                }}, null, 2),
                VECTOR: JSON.stringify({{
                    vector: [0.15, 0.82, -0.34, 0.91, 0.05, 0.44],
                    metadata: {{
                        label: "Embedding de texto LLM",
                        dimension: 6,
                        modelo: "jettra-embed-v1"
                    }}
                }}, null, 2),
                GRAPH: JSON.stringify({{
                    label: "Usuario",
                    properties: {{
                        nombre: "Carlos Gómez",
                        ciudad: "Panamá",
                        edad: 32
                    }},
                    out_edges: [
                        {{ target: "nodo_tecnologia", type: "INTERESADO_EN", weight: 1.0 }}
                    ]
                }}, null, 2),
                TIMESERIES: JSON.stringify({{
                    sensor: "sensor_temperatura_datacenter",
                    valor: 23.8,
                    unidad: "Celsius",
                    ubicacion: "Rack-A03"
                }}, null, 2),
                COLUMN: JSON.stringify({{
                    "perfil:nombre": "Carlos Gómez",
                    "perfil:email": "carlos@jettrardb.io",
                    "cuenta:plan": "Enterprise",
                    "seguridad:2fa": true
                }}, null, 2),
                KEYVALUE: JSON.stringify({{
                    valor: "Cadena o payload binario de ultra alto rendimiento en MemTable"
                }}, null, 2),
                GEOSPATIAL: JSON.stringify({{
                    lat: 8.9824,
                    lon: -79.5199,
                    metadata: {{
                        nombre: "Canal de Panamá",
                        tipo: "Infraestructura",
                        ciudad: "Ciudad de Panamá"
                    }}
                }}, null, 2),
                OBJECT: JSON.stringify({{
                    _class: "UserProfile",
                    state: {{
                        username: "carlos",
                        rol: "ADMIN",
                        preferencias: {{ tema: "dark", notificaciones: true }}
                    }}
                }}, null, 2),
                RECORDS: JSON.stringify({{
                    _recordClass: "EmpleadoRecord",
                    components: {{
                        codigo: "EMP-042",
                        salario: 4500.00,
                        cargo: "Principal Engineer"
                    }},
                    _schema: {{
                        version: 1,
                        strict: true
                    }}
                }}, null, 2)
            }};

            async function getAuthToken() {{
                let token = localStorage.getItem("jettra_token");
                if (!token) {{
                    try {{
                        const res = await fetch("/api/auth/login", {{
                            method: "POST",
                            headers: {{ "Content-Type": "application/json" }},
                            body: JSON.stringify({{ username: "admin", password: "admin" }})
                        }});
                        if (res.ok) {{
                            const data = await res.json();
                            token = data.token;
                            localStorage.setItem("jettra_token", token);
                        }}
                    }} catch (e) {{
                        console.error("Auto login error:", e);
                    }}
                }}
                return token || "";
            }}

            function escapeHtml(str) {{
                if (typeof str !== "string") str = String(str);
                return str.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;").replace(/'/g, "&#039;");
            }}

            function getEngineBadge(engine) {{
                const eng = (engine || "DOCUMENT").toUpperCase();
                switch (eng) {{
                    case "DOCUMENT": return '<span class="badge bg-primary">DOCUMENT</span>';
                    case "VECTOR": return '<span class="badge" style="background-color: #6366f1;">VECTOR</span>';
                    case "GRAPH": return '<span class="badge bg-success">GRAPH</span>';
                    case "TIMESERIES": return '<span class="badge bg-info text-dark">TIMESERIES</span>';
                    case "COLUMN": return '<span class="badge bg-warning text-dark">COLUMN</span>';
                    case "KEYVALUE": return '<span class="badge bg-secondary">KEYVALUE</span>';
                    case "GEOSPATIAL": return '<span class="badge" style="background-color: #0d9488;">GEOSPATIAL</span>';
                    case "OBJECT": return '<span class="badge bg-danger">OBJECT</span>';
                    case "RECORDS": return '<span class="badge" style="background-color: #ea580c;">RECORDS</span>';
                    default: return '<span class="badge bg-dark">' + escapeHtml(eng) + '</span>';
                }}
            }}

            function changeDatabase(db) {{
                window.location.href = "/explorer?db=" + encodeURIComponent(db);
            }}

            function switchView(mode) {{
                currentView = mode;
                if (mode === "table") {{
                    document.getElementById("btnViewTable").classList.add("active");
                    document.getElementById("btnViewTree").classList.remove("active");
                    document.getElementById("tableViewSection").classList.remove("d-none");
                    document.getElementById("treeViewSection").classList.add("d-none");
                }} else {{
                    document.getElementById("btnViewTable").classList.remove("active");
                    document.getElementById("btnViewTree").classList.add("active");
                    document.getElementById("tableViewSection").classList.add("d-none");
                    document.getElementById("treeViewSection").classList.remove("d-none");
                    renderTreeView();
                }}
            }}

            function filterByEngine(engine) {{
                currentFilterEngine = engine;
                const pills = document.querySelectorAll("#engineFilterPills .pill-filter");
                pills.forEach(p => {{
                    if (p.innerText === engine) p.classList.add("active");
                    else p.classList.remove("active");
                }});
                applyFilters();
            }}

            function applyFilters() {{
                currentFilterText = document.getElementById("filterInput").value.trim().toLowerCase();
                renderTableView();
                if (currentView === "tree") {{
                    renderTreeView();
                }}
            }}

            function clearFilter() {{
                document.getElementById("filterInput").value = "";
                currentFilterText = "";
                filterByEngine("ALL");
            }}

            function getFilteredRecords() {{
                return ALL_RECORDS.filter(r => {{
                    if (currentFilterEngine !== "ALL" && r.engine.toUpperCase() !== currentFilterEngine) {{
                        return false;
                    }}
                    if (currentFilterText) {{
                        const text = (r.key + " " + r.namespace + " " + r.id + " " + r.payload).toLowerCase();
                        if (!text.includes(currentFilterText)) return false;
                    }}
                    return true;
                }});
            }}

            function renderTableView() {{
                const filtered = getFilteredRecords();
                const tbody = document.getElementById("tableBody");
                const emptyMsg = document.getElementById("tableEmptyMessage");
                const countBadge = document.getElementById("recordCountBadge");

                countBadge.innerText = filtered.length + " de " + ALL_RECORDS.length + " registros";

                if (filtered.length === 0) {{
                    tbody.innerHTML = "";
                    emptyMsg.classList.remove("d-none");
                    return;
                }}

                emptyMsg.classList.add("d-none");
                let html = "";
                for (let i = 0; i < filtered.length; i++) {{
                    const r = filtered[i];
                    const originalIdx = ALL_RECORDS.indexOf(r);
                    html += '<tr>' +
                        '<td>' + getEngineBadge(r.engine) + '</td>' +
                        '<td><code class="text-white">' + escapeHtml(r.key) + '</code></td>' +
                        '<td><span class="badge bg-dark border border-secondary">' + escapeHtml(r.namespace) + '</span></td>' +
                        '<td><strong class="text-info">' + escapeHtml(r.id) + '</strong></td>' +
                        '<td><div class="payload-preview text-secondary">' + escapeHtml(r.payload) + '</div></td>' +
                        '<td><span class="badge bg-secondary">' + r.version_count + ' v</span></td>' +
                        '<td>' +
                            '<button class="btn btn-xs btn-outline-info me-1 py-1 px-2" onclick="openViewModal(' + originalIdx + ')"><i class="bi bi-eye"></i> Ver</button>' +
                            '<button class="btn btn-xs btn-outline-warning me-1 py-1 px-2" onclick="openEditModal(' + originalIdx + ')"><i class="bi bi-pencil"></i> Editar</button>' +
                            '<button class="btn btn-xs btn-outline-secondary me-1 py-1 px-2" onclick="openHistoryModal(\'' + escapeHtml(r.key) + '\', \'' + escapeHtml(r.namespace) + '\', \'' + escapeHtml(r.id) + '\')"><i class="bi bi-clock-history"></i></button>' +
                            '<button class="btn btn-xs btn-outline-danger py-1 px-2" onclick="deleteRecord(\'' + escapeHtml(r.engine) + '\', \'' + escapeHtml(r.namespace) + '\', \'' + escapeHtml(r.id) + '\')"><i class="bi bi-trash"></i></button>' +
                        '</td>' +
                    '</tr>';
                }}
                tbody.innerHTML = html;
            }}

            function renderJsonNode(val, path) {{
                if (val === null) return '<span class="text-secondary">null</span>';
                if (typeof val === "boolean") return '<span class="text-info">' + val + '</span>';
                if (typeof val === "number") return '<span class="text-warning">' + val + '</span>';
                if (typeof val === "string") return '<span class="text-success">"' + escapeHtml(val) + '"</span>';
                if (Array.isArray(val)) {{
                    if (val.length === 0) return '<span class="text-secondary">[]</span>';
                    let html = '<span class="badge bg-secondary me-1">Array[' + val.length + ']</span><ul class="list-unstyled ps-3 border-start border-secondary mt-1">';
                    for (let i = 0; i < val.length; i++) {{
                        html += '<li class="my-1"><span class="text-secondary small me-1">[' + i + ']:</span> ' + renderJsonNode(val[i], path + "[" + i + "]") + '</li>';
                    }}
                    html += '</ul>';
                    return html;
                }}
                if (typeof val === "object") {{
                    const keys = Object.keys(val);
                    if (keys.length === 0) return '<span class="text-secondary">{{}}</span>';
                    let html = '<span class="badge bg-dark border border-secondary me-1">Object{{' + keys.length + '}}</span><ul class="list-unstyled ps-3 border-start border-secondary mt-1">';
                    for (let k of keys) {{
                        html += '<li class="my-1"><span class="text-light fw-bold me-1">' + escapeHtml(k) + ':</span> ' + renderJsonNode(val[k], path + "." + k) + '</li>';
                    }}
                    html += '</ul>';
                    return html;
                }}
                return escapeHtml(String(val));
            }}

            function renderTreeView() {{
                const filtered = getFilteredRecords();
                const rootEl = document.getElementById("treeRoot");
                if (filtered.length === 0) {{
                    rootEl.innerHTML = '<li class="text-secondary py-3">No hay registros para mostrar en la estructura de árbol.</li>';
                    return;
                }}

                // Agrupar por: Motor -> Namespace -> Registros
                const groups = {{}};
                for (let i = 0; i < filtered.length; i++) {{
                    const r = filtered[i];
                    const eng = r.engine || "DOCUMENT";
                    const ns = r.namespace || "default";
                    if (!groups[eng]) groups[eng] = {{}};
                    if (!groups[eng][ns]) groups[eng][ns] = [];
                    groups[eng][ns].push({{ item: r, originalIdx: ALL_RECORDS.indexOf(r) }});
                }}

                let html = '<li class="tree-node">' +
                    '<span class="tree-toggler open fw-bold text-warning" onclick="toggleTree(this)">' +
                        '<i class="bi bi-database me-1"></i> Base de Datos: ' + escapeHtml(ACTIVE_DB) + ' (' + filtered.length + ' registros)' +
                    '</span>' +
                    '<ul class="tree-children open list-unstyled">';

                for (let eng in groups) {{
                    let engCount = 0;
                    for (let ns in groups[eng]) engCount += groups[eng][ns].length;

                    html += '<li class="tree-node">' +
                        '<span class="tree-toggler open fw-bold text-white" onclick="toggleTree(this)">' +
                            getEngineBadge(eng) + ' <span class="ms-1 text-light">' + escapeHtml(eng) + '</span> <span class="badge bg-secondary small ms-1">' + engCount + '</span>' +
                        '</span>' +
                        '<ul class="tree-children open list-unstyled">';

                    for (let ns in groups[eng]) {{
                        const list = groups[eng][ns];
                        html += '<li class="tree-node">' +
                            '<span class="tree-toggler open text-light" onclick="toggleTree(this)">' +
                                '<i class="bi bi-folder2 text-warning me-1"></i> <strong>' + escapeHtml(ns) + '</strong> <span class="badge bg-dark border border-secondary small ms-1">' + list.length + '</span>' +
                            '</span>' +
                            '<ul class="tree-children open list-unstyled">';

                        for (let itemObj of list) {{
                            const r = itemObj.item;
                            let parsed = {{}};
                            try {{
                                parsed = JSON.parse(r.payload);
                            }} catch(e) {{
                                parsed = {{ raw_payload: r.payload }};
                            }}

                            html += '<li class="tree-node">' +
                                '<div class="d-flex align-items-center flex-wrap gap-2 py-1">' +
                                    '<span class="tree-toggler text-info" onclick="toggleTree(this)">' +
                                        '<i class="bi bi-file-earmark-code me-1 text-info"></i><strong>' + escapeHtml(r.id) + '</strong>' +
                                    '</span>' +
                                    '<span class="badge bg-secondary" style="font-size: 0.7rem;">' + r.version_count + 'v</span>' +
                                    '<div class="btn-group btn-group-xs ms-2">' +
                                        '<button class="btn btn-xs btn-outline-info py-0 px-1" onclick="openViewModal(' + itemObj.originalIdx + ')"><i class="bi bi-eye"></i></button>' +
                                        '<button class="btn btn-xs btn-outline-warning py-0 px-1" onclick="openEditModal(' + itemObj.originalIdx + ')"><i class="bi bi-pencil"></i></button>' +
                                        '<button class="btn btn-xs btn-outline-secondary py-0 px-1" onclick="openHistoryModal(\'' + escapeHtml(r.key) + '\', \'' + escapeHtml(r.namespace) + '\', \'' + escapeHtml(r.id) + '\')"><i class="bi bi-clock-history"></i></button>' +
                                        '<button class="btn btn-xs btn-outline-danger py-0 px-1" onclick="deleteRecord(\'' + escapeHtml(r.engine) + '\', \'' + escapeHtml(r.namespace) + '\', \'' + escapeHtml(r.id) + '\')"><i class="bi bi-trash"></i></button>' +
                                    '</div>' +
                                '</div>' +
                                '<div class="tree-children ps-3 border-start border-secondary mb-2">' +
                                    renderJsonNode(parsed, r.id) +
                                '</div>' +
                            '</li>';
                        }}

                        html += '</ul></li>';
                    }}

                    html += '</ul></li>';
                }}

                html += '</ul></li>';
                rootEl.innerHTML = html;
            }}

            function toggleTree(element) {{
                element.classList.toggle("open");
                const nextSibling = element.nextElementSibling;
                if (nextSibling && (nextSibling.classList.contains("tree-children") || nextSibling.tagName === "UL")) {{
                    nextSibling.classList.toggle("open");
                }}
            }}

            function expandAllTree() {{
                document.querySelectorAll(".tree-toggler").forEach(el => el.classList.add("open"));
                document.querySelectorAll(".tree-children").forEach(el => el.classList.add("open"));
            }}

            function collapseAllTree() {{
                document.querySelectorAll(".tree-toggler").forEach(el => el.classList.remove("open"));
                document.querySelectorAll(".tree-children").forEach(el => el.classList.remove("open"));
            }}

            function openViewModal(index) {{
                const r = ALL_RECORDS[index];
                if (!r) return;
                document.getElementById("viewModalEngineBadge").innerHTML = getEngineBadge(r.engine);
                document.getElementById("viewModalId").innerText = r.id;
                document.getElementById("viewModalNs").innerText = r.namespace;
                document.getElementById("viewModalVersions").innerText = r.version_count + " versiones registradas";
                document.getElementById("viewModalKey").innerText = r.key;

                let pretty = r.payload;
                try {{
                    const parsed = JSON.parse(r.payload);
                    pretty = JSON.stringify(parsed, null, 2);
                }} catch (e) {{}}
                document.getElementById("viewModalPayload").innerText = pretty;

                const modal = new bootstrap.Modal(document.getElementById("viewRecordModal"));
                modal.show();
            }}

            function copyViewJson() {{
                const text = document.getElementById("viewModalPayload").innerText;
                navigator.clipboard.writeText(text);
                alert("JSON copiado al portapapeles");
            }}

            function openCreateModal() {{
                document.getElementById("createRecordError").classList.add("d-none");
                document.getElementById("createModalId").value = "rec_" + Math.floor(Math.random() * 9000 + 1000);
                onModelSelectChange();
                const modal = new bootstrap.Modal(document.getElementById("createRecordModal"));
                modal.show();
            }}

            function onModelSelectChange() {{
                const model = document.getElementById("createModalModel").value;
                const template = ENGINE_TEMPLATES[model] || "{{}}";
                document.getElementById("createModalPayload").value = template;
            }}

            function resetModelTemplate() {{
                onModelSelectChange();
            }}

            async function submitCreateRecord() {{
                const model = document.getElementById("createModalModel").value;
                const ns = document.getElementById("createModalNs").value.trim() || ACTIVE_DB;
                const id = document.getElementById("createModalId").value.trim();
                const payloadStr = document.getElementById("createModalPayload").value.trim();
                const errDiv = document.getElementById("createRecordError");
                errDiv.classList.add("d-none");

                if (!id) {{
                    errDiv.innerText = "Por favor especifique un ID para el registro";
                    errDiv.classList.remove("d-none");
                    return;
                }}

                if (model !== "KEYVALUE") {{
                    try {{
                        JSON.parse(payloadStr);
                    }} catch (e) {{
                        errDiv.innerText = "El contenido debe ser un JSON válido: " + e.message;
                        errDiv.classList.remove("d-none");
                        return;
                    }}
                }}

                const token = await getAuthToken();
                try {{
                    const url = "/api/model/" + encodeURIComponent(model.toLowerCase()) + "/" + encodeURIComponent(ns) + "/" + encodeURIComponent(id);
                    const res = await fetch(url, {{
                        method: "POST",
                        headers: {{
                            "Content-Type": "application/json",
                            "Authorization": "Bearer " + token
                        }},
                        body: payloadStr
                    }});

                    if (res.ok || res.status === 201) {{
                        window.location.reload();
                    }} else {{
                        const data = await res.json().catch(() => ({{}}));
                        errDiv.innerText = data.error || ("Error al guardar registro (HTTP " + res.status + ")");
                        errDiv.classList.remove("d-none");
                    }}
                }} catch (e) {{
                    errDiv.innerText = "Error de conexión: " + e.message;
                    errDiv.classList.remove("d-none");
                }}
            }}

            function openEditModal(index) {{
                const r = ALL_RECORDS[index];
                if (!r) return;
                document.getElementById("editModalModel").value = r.engine;
                document.getElementById("editModalNs").value = r.namespace;
                document.getElementById("editModalId").value = r.id;
                document.getElementById("editRecordError").classList.add("d-none");

                let pretty = r.payload;
                try {{
                    const parsed = JSON.parse(r.payload);
                    pretty = JSON.stringify(parsed, null, 2);
                }} catch (e) {{}}
                document.getElementById("editModalPayload").value = pretty;

                const modal = new bootstrap.Modal(document.getElementById("editRecordModal"));
                modal.show();
            }}

            async function submitEditRecord() {{
                const model = document.getElementById("editModalModel").value;
                const ns = document.getElementById("editModalNs").value;
                const id = document.getElementById("editModalId").value;
                const payloadStr = document.getElementById("editModalPayload").value.trim();
                const errDiv = document.getElementById("editRecordError");
                errDiv.classList.add("d-none");

                if (model !== "KEYVALUE") {{
                    try {{
                        JSON.parse(payloadStr);
                    }} catch (e) {{
                        errDiv.innerText = "El contenido debe ser un JSON válido: " + e.message;
                        errDiv.classList.remove("d-none");
                        return;
                    }}
                }}

                const token = await getAuthToken();
                try {{
                    const url = "/api/model/" + encodeURIComponent(model.toLowerCase()) + "/" + encodeURIComponent(ns) + "/" + encodeURIComponent(id);
                    const res = await fetch(url, {{
                        method: "PUT",
                        headers: {{
                            "Content-Type": "application/json",
                            "Authorization": "Bearer " + token
                        }},
                        body: payloadStr
                    }});

                    if (res.ok || res.status === 200 || res.status === 201) {{
                        window.location.reload();
                    }} else {{
                        const data = await res.json().catch(() => ({{}}));
                        errDiv.innerText = data.error || ("Error al actualizar registro (HTTP " + res.status + ")");
                        errDiv.classList.remove("d-none");
                    }}
                }} catch (e) {{
                    errDiv.innerText = "Error de conexión: " + e.message;
                    errDiv.classList.remove("d-none");
                }}
            }}

            async function deleteRecord(model, ns, id) {{
                if (!confirm("¿Está seguro de que desea eliminar el registro '" + id + "' del motor '" + model + "'?")) {{
                    return;
                }}
                const token = await getAuthToken();
                try {{
                    const url = "/api/model/" + encodeURIComponent(model.toLowerCase()) + "/" + encodeURIComponent(ns) + "/" + encodeURIComponent(id);
                    const res = await fetch(url, {{
                        method: "DELETE",
                        headers: {{ "Authorization": "Bearer " + token }}
                    }});
                    if (res.ok || res.status === 204) {{
                        window.location.reload();
                    }} else {{
                        alert("Error al eliminar registro (HTTP " + res.status + ")");
                    }}
                }} catch (e) {{
                    alert("Error de conexión: " + e.message);
                }}
            }}

            async function openHistoryModal(key, ns, id) {{
                document.getElementById("historyModalKey").innerText = key;
                const tbody = document.getElementById("historyTableBody");
                tbody.innerHTML = '<tr><td colspan="5" class="text-center py-3"><div class="spinner-border spinner-border-sm text-warning me-2"></div>Cargando versiones MVCC...</td></tr>';

                const modal = new bootstrap.Modal(document.getElementById("historyRecordModal"));
                modal.show();

                const token = await getAuthToken();
                try {{
                    const url = "/api/document/" + encodeURIComponent(ns) + "/" + encodeURIComponent(id) + "/history";
                    const res = await fetch(url, {{
                        headers: {{ "Authorization": "Bearer " + token }}
                    }});

                    if (res.ok) {{
                        const versions = await res.json();
                        if (versions.length === 0) {{
                            tbody.innerHTML = '<tr><td colspan="5" class="text-center text-secondary py-3">No hay historial de versiones para este registro.</td></tr>';
                            return;
                        }}
                        let html = "";
                        for (let v of versions) {{
                            const statusBadge = v.is_current ? '<span class="badge bg-success">ACTUAL</span>' : '<span class="badge bg-secondary">PREVIA</span>';
                            const restoreBtn = v.is_current ? '' : '<button class="btn btn-xs btn-outline-warning py-0 px-2" onclick="restoreVersion(\'' + escapeHtml(ns) + '\', \'' + escapeHtml(id) + '\', ' + v.timestamp + ')"><i class="bi bi-arrow-counterclockwise"></i> Restaurar</button>';
                            html += '<tr>' +
                                '<td><span class="badge bg-dark border border-secondary">v' + v.version_number + '</span></td>' +
                                '<td class="small">' + escapeHtml(v.formatted_date) + '</td>' +
                                '<td><div class="payload-preview text-secondary">' + escapeHtml(v.payload) + '</div></td>' +
                                '<td>' + statusBadge + '</td>' +
                                '<td>' + restoreBtn + '</td>' +
                            '</tr>';
                        }}
                        tbody.innerHTML = html;
                    }} else {{
                        tbody.innerHTML = '<tr><td colspan="5" class="text-center text-danger py-3">No se pudo cargar el historial (HTTP ' + res.status + ')</td></tr>';
                    }}
                }} catch (e) {{
                    tbody.innerHTML = '<tr><td colspan="5" class="text-center text-danger py-3">Error: ' + escapeHtml(e.message) + '</td></tr>';
                }}
            }}

            async function restoreVersion(ns, id, timestamp) {{
                if (!confirm("¿Desea restaurar esta versión histórica? Se creará una nueva versión MVCC con este contenido.")) {{
                    return;
                }}
                const token = await getAuthToken();
                try {{
                    const url = "/api/document/" + encodeURIComponent(ns) + "/" + encodeURIComponent(id) + "/restore?timestamp=" + timestamp;
                    const res = await fetch(url, {{
                        method: "POST",
                        headers: {{ "Authorization": "Bearer " + token }}
                    }});
                    if (res.ok) {{
                        alert("Versión restaurada con éxito");
                        window.location.reload();
                    }} else {{
                        alert("Error al restaurar versión");
                    }}
                }} catch (e) {{
                    alert("Error: " + e.message);
                }}
            }}

            // Renderizar inicial
            renderTableView();
        </script>
        "###,
            db_options = db_options,
            active_db = active_db,
            total_records = records.len(),
            records_json = records_json
        );

        Self::layout("Data Explorer", "explorer", &content)
    }

    pub fn engines_page() -> String {
        let content = r#"
        <div class="d-flex justify-content-between align-items-center pt-2 pb-3 mb-4 border-bottom border-secondary">
            <h1 class="h2"><i class="bi bi-cpu text-warning me-2"></i>The 9 Multi-Model Engines Workbench</h1>
        </div>

        <div class="row g-4">
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">1</span>Document Engine</h5>
                    <p class="text-secondary small">JSON/NoSQL tree storage with automated UUID/autoincrement ID generation, version history and point-in-time recovery.</p>
                    <div class="code-box mb-3">POST /api/document/{collection}/{id}<br>GET /api/document/{collection}/{id}/history</div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">2</span>Vector Engine</h5>
                    <p class="text-secondary small">High-dimensional embeddings, Cosine Similarity and Approximate Nearest Neighbor (ANN) search.</p>
                    <div class="code-box mb-3">POST /api/model/vector/{collection}/{id}<br>Payload: {"vector": [0.1, 0.2, 0.3], "metadata": {}}</div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">3</span>Graph Engine</h5>
                    <p class="text-secondary small">Labeled Property Graph (LPG), directed edges, and deep relational graph traversal.</p>
                    <div class="code-box mb-3">POST /api/model/graph/{graphId}/{nodeId}<br>Key: graph:{graphId}:edge:{from}:{to}:{label}</div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">4</span>TimeSeries Engine</h5>
                    <p class="text-secondary small">Temporal data ingestion with microsecond sorting and downsampling aggregations.</p>
                    <div class="code-box mb-3">POST /api/model/timeseries/{measurement}/{timestamp}<br>Payload: {"cpu": 42.5, "mem": 1024}</div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">5</span>Column Engine</h5>
                    <p class="text-secondary small">OLAP columnar storage, row keys, and fast columnar projections.</p>
                    <div class="code-box mb-3">POST /api/model/column/{family}/{rowKey}<br>Payload: {"cf:status": "active", "cf:score": 99}</div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">6</span>KeyValue Engine</h5>
                    <p class="text-secondary small">High-speed atomic string and binary key-value caching directly in the MemTable.</p>
                    <div class="code-box mb-3">POST /api/model/keyvalue/{namespace}/{key}<br>Body: raw string or bytes</div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">7</span>Geospatial Engine</h5>
                    <p class="text-secondary small">2D GPS coordinates with Haversine distance proximity queries and radius filtering.</p>
                    <div class="code-box mb-3">POST /api/model/geospatial/{collection}/{locId}<br>Payload: {"lat": 8.98, "lon": -79.52}</div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">8</span>Object Engine</h5>
                    <p class="text-secondary small">Serialized object BLOBs, class state metadata, and streaming binary storage.</p>
                    <div class="code-box mb-3">POST /api/model/object/{collection}/{id}<br>Payload: {"_class": "Product", "state": {...}}</div>
                </div>
            </div>
            <div class="col-md-6">
                <div class="card card-custom p-4 h-100">
                    <h5 class="card-title text-white"><span class="badge badge-engine me-2">9</span>Records Engine</h5>
                    <p class="text-secondary small">Strongly typed schema validation, component introspection and field projection (?fields=a,b).</p>
                    <div class="code-box mb-3">POST /api/model/records/{collection}/{id}<br>GET /api/model/records/{collection}/{id}?fields=name,salary</div>
                </div>
            </div>
        </div>
        "#;

        Self::layout("Engines", "engines", content)
    }

    pub fn users_page() -> String {
        let content = r#"
        <div class="d-flex justify-content-between align-items-center pt-2 pb-3 mb-4 border-bottom border-secondary">
            <h1 class="h2"><i class="bi bi-people text-warning me-2"></i>Security & RBAC Management</h1>
        </div>

        <div class="card card-custom p-4">
            <h5 class="card-title text-white mb-3">Default Administrative Accounts</h5>
            <div class="table-responsive">
                <table class="table table-dark table-hover align-middle">
                    <thead>
                        <tr>
                            <th>Username</th>
                            <th>Role</th>
                            <th>Accessible Databases</th>
                            <th>Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="fw-bold text-white"><i class="bi bi-person-fill text-warning me-2"></i>admin</td>
                            <td><span class="badge bg-primary">ADMIN</span></td>
                            <td><code>* (All)</code></td>
                            <td><span class="badge bg-success">ACTIVE</span></td>
                        </tr>
                        <tr>
                            <td class="fw-bold text-white"><i class="bi bi-person-fill text-warning me-2"></i>super-user</td>
                            <td><span class="badge bg-danger">SUPERUSER</span></td>
                            <td><code>* (All)</code></td>
                            <td><span class="badge bg-warning text-dark">PASSWORD CHANGE REQUIRED</span></td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        "#;

        Self::layout("Users & RBAC", "users", content)
    }

    pub fn components_page(node_id: &str, grpc_port: u16, rest_port: u16, peers: &str) -> String {
        let content = format!(r#"
        <div class="d-flex justify-content-between align-items-center pt-2 pb-3 mb-4 border-bottom border-secondary">
            <h1 class="h2"><i class="bi bi-diagram-3 text-warning me-2"></i>Cluster Topology & Consensus</h1>
        </div>

        <div class="row g-4">
            <div class="col-lg-6">
                <div class="card card-custom p-4">
                    <h5 class="card-title text-white mb-3"><i class="bi bi-server text-warning me-2"></i>Local Node Configuration</h5>
                    <ul class="list-unstyled text-secondary">
                        <li class="mb-2"><strong class="text-white">Node Identifier:</strong> {node_id}</li>
                        <li class="mb-2"><strong class="text-white">REST Database Port:</strong> {rest_port}</li>
                        <li class="mb-2"><strong class="text-white">Raft Consensus Port:</strong> {grpc_port}</li>
                        <li class="mb-2"><strong class="text-white">Role:</strong> <span class="badge bg-success">LEADER / ACTIVE</span></li>
                    </ul>
                </div>
            </div>
            <div class="col-lg-6">
                <div class="card card-custom p-4">
                    <h5 class="card-title text-white mb-3"><i class="bi bi-people text-warning me-2"></i>Configured Raft Peers</h5>
                    <div class="code-box">{peers}</div>
                </div>
            </div>
        </div>
        "#, node_id = node_id, grpc_port = grpc_port, rest_port = rest_port, peers = peers);

        Self::layout("Components", "components", &content)
    }

    pub fn information_page() -> String {
        let content = r#"
        <div class="d-flex justify-content-between align-items-center pt-2 pb-3 mb-4 border-bottom border-secondary">
            <h1 class="h2"><i class="bi bi-info-circle text-warning me-2"></i>JettraRDB Architecture Information</h1>
        </div>

        <div class="card card-custom p-4 mb-4">
            <h4 class="text-white"><i class="bi bi-cpu text-warning me-2"></i>The Rust Advantage over JVM</h4>
            <p class="text-secondary">JettraRDB transforms the original Java 25 JettraDB architecture into a native, zero-GC, ultra-high throughput database system engineered with Rust:</p>
            <div class="row g-3">
                <div class="col-md-4">
                    <div class="p-3 border border-secondary rounded bg-dark">
                        <h6 class="text-warning">Zero Garbage Collection</h6>
                        <p class="small text-secondary mb-0">Deterministic memory management via Rust ownership guarantees no GC stop-the-world pauses, achieving microsecond tail latency.</p>
                    </div>
                </div>
                <div class="col-md-4">
                    <div class="p-3 border border-secondary rounded bg-dark">
                        <h6 class="text-warning">Minimal Footprint</h6>
                        <p class="small text-secondary mb-0">Single standalone native binary without needing JRE/JDK runtimes. Docker images under 25 MB using Alpine or distroless containers.</p>
                    </div>
                </div>
                <div class="col-md-4">
                    <div class="p-3 border border-secondary rounded bg-dark">
                        <h6 class="text-warning">Asynchronous Concurrency</h6>
                        <p class="small text-secondary mb-0">Tokio event-loop handles thousands of simultaneous connections with low memory consumption and asynchronous I/O.</p>
                    </div>
                </div>
            </div>
        </div>
        "#;

        Self::layout("Information", "information", content)
    }

    pub fn swagger_ui_page(rest_port: u16) -> String {
        format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>JettraRDB Swagger / OpenAPI Explorer</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui.css" />
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui-bundle.js"></script>
    <script>
        window.onload = () => {{
            window.ui = SwaggerUIBundle({{
                spec: {{
                    openapi: "3.0.0",
                    info: {{
                        title: "JettraRDB REST API",
                        version: "1.0.0",
                        description: "Autonomous Multi-Model Database Engine in Rust (9 Database Models)"
                    }},
                    servers: [{{ url: "http://localhost:{rest_port}" }}],
                    paths: {{
                        "/api/auth/login": {{
                            post: {{
                                summary: "Authenticate and get Bearer Token",
                                requestBody: {{
                                    content: {{
                                        "application/json": {{
                                            schema: {{
                                                type: "object",
                                                properties: {{
                                                    username: {{ type: "string", example: "admin" }},
                                                    password: {{ type: "string", example: "admin" }}
                                                }}
                                            }}
                                        }}
                                    }}
                                }},
                                responses: {{
                                    "200": {{ description: "Token returned successfully" }}
                                }}
                            }}
                        }},
                        "/api/model/{{model}}/{{namespace}}/{{id}}": {{
                            post: {{
                                summary: "Universal Multi-Model Insert (document, vector, graph, timeseries, column, keyvalue, geospatial, object, records)",
                                parameters: [
                                    {{ name: "model", in: "path", required: true, schema: {{ type: "string" }} }},
                                    {{ name: "namespace", in: "path", required: true, schema: {{ type: "string" }} }},
                                    {{ name: "id", in: "path", required: true, schema: {{ type: "string" }} }}
                                ],
                                responses: {{ "201": {{ description: "Stored successfully" }} }}
                            }},
                            get: {{
                                summary: "Universal Multi-Model Get",
                                parameters: [
                                    {{ name: "model", in: "path", required: true, schema: {{ type: "string" }} }},
                                    {{ name: "namespace", in: "path", required: true, schema: {{ type: "string" }} }},
                                    {{ name: "id", in: "path", required: true, schema: {{ type: "string" }} }}
                                ],
                                responses: {{ "200": {{ description: "Model data returned" }} }}
                            }},
                            delete: {{
                                summary: "Universal Multi-Model Delete",
                                parameters: [
                                    {{ name: "model", in: "path", required: true, schema: {{ type: "string" }} }},
                                    {{ name: "namespace", in: "path", required: true, schema: {{ type: "string" }} }},
                                    {{ name: "id", in: "path", required: true, schema: {{ type: "string" }} }}
                                ],
                                responses: {{ "204": {{ description: "Deleted" }} }}
                            }}
                        }},
                        "/api/document/{{collection}}/{{id}}": {{
                            get: {{
                                summary: "Get document from collection",
                                parameters: [
                                    {{ name: "collection", in: "path", required: true, schema: {{ type: "string" }} }},
                                    {{ name: "id", in: "path", required: true, schema: {{ type: "string" }} }}
                                ],
                                responses: {{ "200": {{ description: "Document found" }} }}
                            }}
                        }}
                    }}
                }},
                dom_id: '#swagger-ui',
            }});
        }};
    </script>
</body>
</html>"#, rest_port = rest_port)
    }
}

