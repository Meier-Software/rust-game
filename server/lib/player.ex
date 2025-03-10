defmodule Player do
  require Logger

  @moduledoc """
  A player process should be in charge of its own data and should regularly save this state to the db.

  TODO: Kill player when client leaves or logs out.
  """

  def start(client_pid, username) do
    Logger.info("New player-(#{username}) spawned for client-(#{inspect(client_pid)})")

    stats = %{:hp => 10, :mp => 20}

    info = %{
      :x => 0.0,
      :y => 0.0,
      :facing => "North",
      :username => username
    }

    send(:zone_manager, {:player_join, username, self()})
    loop_player(client_pid, stats, info)
  end

  def loop_player(client_pid, stats, info) do
    receive do
      {:heal, value, pid} ->
        {:ok, username} = Map.fetch(info, :username)
        Logger.info("Player #{inspect(username)} healed.")

        {value, _} = Integer.parse(value)
        {:ok, hp} = Map.fetch(stats, :hp)
        stats = Map.put(stats, :hp, hp + value)

        send(pid, {:client_send, "Healed for #{inspect(value)}."})
        loop_player(client_pid, stats, info)

      {:stats, pid} ->
        send(pid, {:stats, stats, info})
        loop_player(client_pid, stats, info)

      {:facing, dir, pid} ->
        send(pid, {:faced, dir})
        info = Map.put(info, :facing, dir)

        loop_player(client_pid, stats, info)

      {:move, x, y, pid} ->
        send(pid, {:moved, x, y})
        {x_delta, _} = Integer.parse(x)
        {y_delta, _} = Integer.parse(y)

        {:ok, x} = Map.fetch(info, :x)
        info = Map.put(info, :x, x + x_delta)

        {:ok, y} = Map.fetch(info, :y)
        info = Map.put(info, :y, y + y_delta)

        loop_player(client_pid, stats, info)

      {:client_send, line} ->
        Logger.info("Player got client send event.")
        send(client_pid, {:client_send, line})
        loop_player(client_pid, stats, info)

      err ->
        {:ok, username} = Map.fetch(info, :username)

        Logger.info("Client Error #{inspect(err)} from #{username}.")
        loop_player(client_pid, stats, info)

      _ ->
        loop_player(client_pid, stats, info)
    after
      0 ->
        loop_player(client_pid, stats, info)
    end
  end
end
