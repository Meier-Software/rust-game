defmodule Client.Authed do
  require Logger

  def start(client_socket, username) do
    Logger.info("New player-(#{username})")
    client_pid = self()

    # TODO: Only actually go through this when auth starts
    {:ok, player_pid} =
      Task.Supervisor.start_child(Server.Clients, fn -> Player.start(client_pid, username) end)

    write_line("Logged in", client_socket)
    loop_client(client_socket, username, player_pid)
  end

  defp process_line(line, client_socket, username, player_pid) do
    cmds = Client.Commands.line_to_commands(line)

    case cmds do
      ["logout"] ->
        Logger.info("Client logged out.")
        Client.repeat_start(client_socket)

      ["quit"] ->
        Logger.info("Client quit.")

        :gen_tcp.close(client_socket)
        Process.exit(self(), "Client quit.")

      ["stats"] ->
        send(player_pid, {:stats, self()})

        receive do
          {:stats, stats, info} -> "#{inspect(stats)} #{inspect(info)}"
        end

      ["heal", value] ->
        send(player_pid, {:heal, value, self()})
        "Attempting to heal for #{value}."

      # BUG: This does not preserve spaces in echo when it probably should.
      ["echo", rest] ->
        Logger.info("#{inspect(rest)}")
        "#{rest}"

      ["move", x, y] ->
        send(player_pid, {:move, x, y, self()})
        "Moved x#{inspect(x)} y#{inspect(y)}"

      ["pos", x, y] ->
        send(player_pid, {:move, x, y, self()})
        "Position set to x#{inspect(x)} y#{inspect(y)}"

      ["face", dir] ->
        send(player_pid, {:facing, dir, self()})
        "Facing " <> dir

      ["username", username] ->
        # Logger.info("Client identified as username: #{username}")
        send(player_pid, {:set_username, username, self()})
        "Username set to " <> username

      ["chat", message] ->
        # Combine all parts of the message
        full_message = Enum.join([message], " ")
        Logger.info("Chat message from #{username}: #{full_message}")

        # Send the chat message to the zone manager to broadcast to all players
        send(:zone_manager, {:broadcast, "chat_message #{username} #{full_message}"})

        # Return a confirmation
        "Chat message sent"

      ["update", "server"] ->
        Logger.info("Recompiling server.")
        Server.recompile()
        Logger.info("Restarting core parts of the server.")
        "Server updated."

      ["help"] ->
        Logger.info("Help.")
        "You used the help command."

      ["help", command] ->
        Logger.info("Help.")
        "You used the help command to get info about #{command}."

      abc ->
        Logger.info("unimplemented " <> inspect(abc))
        "Invalid protocol."
    end
  end

  defp loop_client(client_socket, auth, player_pid) do
    # Logger.info("Loop Client")

    receive do
      {:client_send, line} ->
        # Logger.info("Recieved a :client_send event")
        write_line(line, client_socket)
        loop_client(client_socket, auth, player_pid)

      {:stats, stats} ->
        # Logger.info("stats #{inspect(stats)}")
        line = "stats collected."
        write_line(line, client_socket)
        loop_client(client_socket, auth, player_pid)
    after
      0 ->
        line = read_line(client_socket)
        line = process_line(line, client_socket, auth, player_pid)

        line = "USR-(#{auth}): " <> line
        write_line(line, client_socket)

        loop_client(client_socket, auth, player_pid)
        # code
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

  defp raw_write_line(line, client_socket) do
    :gen_tcp.send(client_socket, line)
  end
end
