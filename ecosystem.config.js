export const apps = [{
  name: 'topacademybot',
  script: 'npm',
  args: 'start',

  cwd: ".",

  instances: 1,
  exec_mode: 'fork',

  autorestart: true,
  watch: false,

  max_memory_restart: '800M',

  error_file: './logs/error.log',
  out_file: './logs/out.log',
  log_date_format: 'YYYY-MM-DD HH:mm:ss',
  merge_logs: true,

  min_uptime: '30s',
  max_restarts: 10,
  restart_delay: 4000
}];