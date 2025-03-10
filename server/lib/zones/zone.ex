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
        
        # Log the current player list for debugging
        Logger.info("Current players before join: #{inspect(Map.keys(player_list))}")
        
        # Send information about existing players to the new player
        for {existing_name, existing_pid} <- player_list do
          Logger.info("Sending existing player info to new player: #{existing_name}")
          # Request position from the existing player
          send(existing_pid, {:get_position, self()})
          
          # Wait for the response
          receive do
            {:position_info, x, y, facing} ->
              # Send the information to the new player
              Logger.info("Sending player_joined message to new player: #{existing_name} at (#{x}, #{y})")
              send(player_pid, {:client_send, "player_joined #{existing_name} #{x} #{y} #{facing}"})
          after
            1000 -> Logger.warn("Timeout waiting for position info from #{existing_name}")
          end
        end
        
        # Now add the new player to the list
        player_list = Map.put(player_list, name, player_pid)
        zone_data = Map.put(zone_data, :playerlist, player_list)
        Logger.info("Player joined: #{name}")
        Logger.info("Current players after join: #{inspect(Map.keys(player_list))}")

        # Broadcast a general message that a player joined
        send(self(), {:broadcast, "Player #{name} joined HUB"})
        
        # Request the new player's position to broadcast to others
        send(player_pid, {:get_position, self()})
        
        # Wait for the response
        receive do
          {:position_info, x, y, facing} ->
            # Broadcast player_joined with position information to all other players
            Logger.info("Broadcasting new player to all existing players: #{name} at (#{x}, #{y})")
            for {k, other_pid} <- player_list, k != name do
              Logger.info("Broadcasting to player #{k}")
              send(other_pid, {:client_send, "player_joined #{name} #{x} #{y} #{facing}"})
            end
        after
          1000 -> Logger.warn("Timeout waiting for position info from new player #{name}")
        end
        
        loop_zone(zone_data)
        
      {:player_moved, username, x, y, facing} ->
        Logger.info("Player moved: #{username} at (#{x}, #{y}) facing #{facing}")
        player_list = Map.get(zone_data, :playerlist)
        Logger.info("Current players: #{inspect(Map.keys(player_list))}")
        
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
        player_list = Map.get(zone_data, :playerlist)

        for {k, player_pid} <- player_list do
          send(player_pid, {:client_send, line})
        end

        loop_zone(zone_data)
        
      {:broadcast_player_joined, username, x, y, facing} ->
        Logger.info("Player position: #{username} at (#{x}, #{y})")
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
