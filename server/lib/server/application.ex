defmodule Server.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  def start(_type, _args) do
    children = [
      # {Task.Supervisor, name: Server.TaskSupervisor},
      {Task.Supervisor, name: Server.Services},
      {Task.Supervisor, name: Server.Clients},
      {Task.Supervisor, name: Server.Players},
      {Task.Supervisor, name: Server.Zones},

      # Starts a worker by calling: Server.Worker.start_link(arg)
      # {Server.Worker, arg}
      Supervisor.child_spec({Task, fn -> Server.start() end}, restart: :temporary)
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Server.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
