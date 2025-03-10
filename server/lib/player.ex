defmodule Player do
  require Logger

  @moduledoc """
  A player process should be in charge of its own data and should regularly save this state to the db.

  TODO: Kill player when client leaves or logs out.
  """

  def start(client_pid, username) do
    Logger.info("Player joined: #{username}")

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

        {value, _} = Integer.parse(value)
        {:ok, hp} = Map.fetch(stats, :hp)
        stats = Map.put(stats, :hp, hp + value)

        send(pid, {:client_send, "Healed for #{inspect(value)}."})
        loop_player(client_pid, stats, info)

      {:stats, pid} ->
        send(pid, {:stats, stats, info})
        loop_player(client_pid, stats, info)

      {:get_position, pid} ->
        # Get current position and facing
        {:ok, x} = Map.fetch(info, :x)
        {:ok, y} = Map.fetch(info, :y)
        {:ok, facing} = Map.fetch(info, :facing)
        
        # Send position info back to the requester
        send(pid, {:position_info, x, y, facing})
        loop_player(client_pid, stats, info)

      {:facing, dir, pid} ->
        send(pid, {:faced, dir})
        info = Map.put(info, :facing, dir)
        
        # Get the username
        {:ok, username} = Map.fetch(info, :username)
        
        # Get current position
        {:ok, x} = Map.fetch(info, :x)
        {:ok, y} = Map.fetch(info, :y)
        
        # Broadcast the facing change to all players in the zone
        send(:zone_manager, {:player_moved, username, x, y, dir})

        loop_player(client_pid, stats, info)
        
      {:set_username, username, pid} ->
        Logger.info("Player joined: #{username}")
        info = Map.put(info, :username, username)
        send(pid, {:client_send, "Username set to #{username}"})
        loop_player(client_pid, stats, info)

      {:move, x, y, pid} ->
        send(pid, {:moved, x, y})
        {x_delta, _} = Integer.parse(x)
        {y_delta, _} = Integer.parse(y)

        # Update position - treat x and y as absolute positions, not deltas
        info = Map.put(info, :x, x_delta)
        info = Map.put(info, :y, y_delta)
        
        # Get the current facing direction
        {:ok, facing} = Map.fetch(info, :facing)
        
        # Get the username
        {:ok, username} = Map.fetch(info, :username)
        
        # Broadcast the movement to all players in the zone
        send(:zone_manager, {:player_moved, username, x_delta, y_delta, facing})

        loop_player(client_pid, stats, info)

      {:client_send, line} ->
        send(client_pid, {:client_send, line})
        loop_player(client_pid, stats, info)

      err ->
        {:ok, username} = Map.fetch(info, :username)
        Logger.warn("Client Error #{inspect(err)} from #{username}.")
        loop_player(client_pid, stats, info)

      _ ->
        loop_player(client_pid, stats, info)
    after
      0 ->
        loop_player(client_pid, stats, info)
    end
  end
end
