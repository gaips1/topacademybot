module.exports = {
  apps: [
    {
      name: "topacademybot-prod",
      script: "npm",
      args: "start",
      cwd: __dirname,
      instances: "1",
      exec_mode: "fork",
      autorestart: true,
      watch: false,
      max_memory_restart: "1G",
    }
  ]
};