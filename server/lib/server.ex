defmodule Server do
  require Logger

  @moduledoc """
  Documentation for `Server`.
  """

  @doc """
  This will act as the Server Coordinator
  """
  def start() do
    Logger.info("Spawning services.")

    # Start the process to handle the new client and plumb it up to the Proxy Task Supervisor which handles errors and restarts of processes.
    {:ok, acceptor_pid} =
      Task.Supervisor.start_child(Server.Services, fn -> Acceptor.start() end)

    Logger.info("Running the acceptor service at #{inspect(acceptor_pid)}")

    {:ok, zone_manager_pid} =
      Task.Supervisor.start_child(Server.Services, fn -> ZoneManager.start() end)

    Logger.info("Running the Zone Manager service at #{inspect(zone_manager_pid)}")

    {:ok, db_pid} =
      Task.Supervisor.start_child(Server.Services, fn -> Database.start() end)

    # Mix.ensure_application!(:observer)
    # {:ok, observer_pid} =
    #   Task.Supervisor.start_child(Server.Services, fn -> :observer.start() end)

    Logger.info("Running the Database service at #{inspect(db_pid)}")

    Process.exit(self(), "Start up finished.")
  end

  def recompile() do
    Mix.Task.reenable("app.start")
    Mix.Task.reenable("compile")
    Mix.Task.reenable("compile.all")
    compilers = Mix.compilers()
    Enum.each(compilers, &Mix.Task.reenable("compile.#{&1}"))
    Mix.Task.run("compile.all")
  end
end
