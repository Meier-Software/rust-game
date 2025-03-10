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
        
        # Send information about existing players to the new player
        for {existing_name, existing_pid} <- player_list do
          Logger.info("Sending existing player info to new player: #{existing_name}")
          # Request position from the existing player
          send(existing_pid, {:get_position, self()})
          
          # Wait for the response
          receive do
            {:position_info, x, y, facing} ->
              # Send the information to the new player
              send(player_pid, {:client_send, "player_joined #{existing_name} #{x} #{y} #{facing}"})
          after
            1000 -> Logger.warn("Timeout waiting for position info from #{existing_name}")
          end
        end
        
        # Now add the new player to the list
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
        Logger.info("Current player list: #{inspect(player_list)}")
        
        # Broadcast to all players except the one who moved
        broadcast_count = 0
        for {k, player_pid} <- player_list, k != username do
          Logger.info("Broadcasting movement to player #{k}")
          send(player_pid, {:client_send, "player_moved #{username} #{x} #{y} #{facing}"})
          broadcast_count = broadcast_count + 1
        end
        
        Logger.info("Movement broadcast to #{broadcast_count} players")
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
        Logger.info("Current player list: #{inspect(player_list)}")
        
        # Broadcast to all players except the one who joined
        broadcast_count = 0
        for {k, player_pid} <- player_list, k != username do
          Logger.info("Broadcasting join to player #{k}")
          send(player_pid, {:client_send, "player_joined #{username} #{x} #{y} #{facing}"})
          broadcast_count = broadcast_count + 1
        end
        
        Logger.info("Join broadcast to #{broadcast_count} players")
        loop_zone(zone_data)
    after
      0 ->
        loop_zone(zone_data)
    end
  end
end
