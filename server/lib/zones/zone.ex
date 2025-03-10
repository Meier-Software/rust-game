defmodule Zone do
  require Logger

  def start(name) do
    Logger.info("Zone #{name} ready to recieve players.")

    player_list =
      %{
        # Name, PID
      }

    zone_data = %{:zonename => "#{name}", :playerlist => player_list}

    loop_zone(zone_data)
  end

  def loop_zone(zone_data) do
    receive do
      {:player_join, name, player_pid} ->
        zonename = Map.get(zone_data, :zonename)
        player_list = Map.get(zone_data, :playerlist)
        player_list = Map.put(player_list, name, player_pid)
        zone_data = Map.put(zone_data, :playerlist, player_list)
        Logger.info("New player #{inspect(name)} joined zone #{inspect(zone_data)}.")

        send(self(), {:broadcast, "Player #{name} joined HUB"})
        
        # Also broadcast player_joined with position information
        send(self(), {:broadcast_player_joined, name, 0, 0, "South"})
        
        loop_zone(zone_data)
        
      {:player_moved, username, x, y, facing} ->
        Logger.info("Player #{username} moved to (#{x}, #{y}) facing #{facing}")
        player_list = Map.get(zone_data, :playerlist)
        
        # Broadcast to all players except the one who moved
        for {k, player_pid} <- player_list, k != username do
          send(player_pid, {:client_send, "player_moved #{username} #{x} #{y} #{facing}"})
        end
        
        loop_zone(zone_data)

      {:broadcast, line} ->
        Logger.info("Attempt broadcast.")
        player_list = Map.get(zone_data, :playerlist)

        for {k, player_pid} <- player_list do
          Logger.info("Player name #{k}, player pid #{inspect(player_pid)}")
          send(player_pid, {:client_send, line})
        end

        loop_zone(zone_data)
        
      {:broadcast_player_joined, username, x, y, facing} ->
        Logger.info("Broadcasting player joined: #{username} at (#{x}, #{y}) facing #{facing}")
        player_list = Map.get(zone_data, :playerlist)
        
        # Broadcast to all players except the one who joined
        for {k, player_pid} <- player_list, k != username do
          send(player_pid, {:client_send, "player_joined #{username} #{x} #{y} #{facing}"})
        end
        
        loop_zone(zone_data)
    after
      0 ->
        loop_zone(zone_data)
    end
  end
end
