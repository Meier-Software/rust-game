defmodule Client do
  require Logger

  @moduledoc """
  This module handles client networking.
  Combined with a "Player" this should fully support a player joining and playing the game.
  """

  def start(client_socket) do
    Logger.info("New client connected.")
    loop_client(client_socket)
  end

  def repeat_start(client_socket) do
    Logger.info("Client relogged in.")
    loop_client(client_socket)
  end

  def loop_client(client_socket) do
    Logger.info "Loop client pre read"
    line = read_line(client_socket)
    Logger.info "Loop client post read"
    line = process_line(line, client_socket)
    write_line(line, client_socket)

    loop_client(client_socket)
  end

  defp process_line(line, client_socket) do
    cmds = Client.Commands.line_to_commands(line)

    case cmds do
      ["login", username, password] ->
        Logger.info("Client logged in.")

        case Db.DbApi.get_player(username) do
          {:player, false} ->
            Logger.info("User #{username} not in the db.")

            "Login error."

          {:player, true} ->
            Client.Authed.start(client_socket, username)
            ""
        end

      ["register", username, password] ->
        Logger.info("Client logged in.")

        case Db.DbApi.get_player(username) do
          {:player, false} ->
            Logger.info("Registering new user #{username}.")
            Db.DbApi.new_player(username, password)
            Client.Authed.start(client_socket, username)

            "Registered user."

          {:player, true} ->
            # Client.Authed.start(client_socket, username)
            "Already a player."
        end

      _ ->
        Logger.info("unimplemented")
        "Invalid protocol."
    end
  end

  defp read_line(client_socket) do
    case :gen_tcp.recv(client_socket, 0) do
      {:ok, data} ->
        data

      {:error, :closed} ->
        # TODO Error handle a broken tcp connection
        Logger.info("Player disconnected from the game.")
        Process.exit(self(), "Player left.")
    end
  end

  defp write_line(line, client_socket) do
    line = "SRV: " <> line <> "\n\r"
    :gen_tcp.send(client_socket, line)
  end

  def raw_write_line(line, client_socket) do
    :gen_tcp.send(client_socket, line)
  end
end
