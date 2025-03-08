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
    loop_client(client_socket)
  end

  def loop_client(client_socket) do
    line = read_line(client_socket)
    line = process_line(line, client_socket)
    write_line(line, client_socket)

    loop_client(client_socket)
  end

  defp process_line(line, client_socket) do
    cmds = Client.Commands.line_to_commands(line)
    Logger.info(inspect(cmds))

    case cmds do
      ["login", username, password] ->
        Logger.info("Client logged in.")

        case Db.DbApi.get_player(username) do
          {:player, false} ->
            Logger.info("User #{username} not in the db.")
            "Login error."

          {:player, true} ->
            Client.Authed.start(client_socket, username)
            "Login Success"
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
