defmodule Acceptor do
  require Logger

  @moduledoc """
  This is the Client connection point.
  From here a Client is spawned which handles auth then
  """
  def start() do
    Logger.info("Client Acceptor spawned.")
    Process.register(self(), :client_acceptor)
    Logger.info("Client Acceptor alias registered.")

    {:ok, socket} =
      :gen_tcp.listen(45250, [:binary, packet: :line, active: false, reuseaddr: true])

    loop_acceptor(socket)
  end

  def loop_acceptor(socket) do
    {:ok, client_socket} = :gen_tcp.accept(socket)

    # Start the process to handle the new client and plumb it up to the Client Task Supervisor which handles errors and restarts of processes.
    {:ok, client_pid} =
      Task.Supervisor.start_child(Server.Clients, fn -> Client.start(client_socket) end)

    # Change the owner of the tcp socket to prevent failure cascade.
    :ok = :gen_tcp.controlling_process(client_socket, client_pid)
    Logger.info("ACCEPTOR: TCP Owner changed hands.")

    loop_acceptor(socket)
  end
end
