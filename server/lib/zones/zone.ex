defmodule Zone do
  require Logger

  def start(name) do
    Logger.info("Zone #{name} ready to recieve players.")

    player_list  = %{
      # Name, PID
    }

    zone_data = %{:zone_name => "#{name}", :playerlist => player_list}

    loop_zone(zone_data)
  end

  def loop_zone(zone_data) do
    receive do
      {:player_join, name, player_pid} ->
        Logger.info "NEW PLAYER #{inspect(name)} JOINED ZONE."
        player_list = Map.get(zone_data, :playerlist)
        player_list = Map.put(player_list, name, player_pid)
        zone_data = Map.put(zone_data, :playerlist, player_list)

        send(self(), {:broadcast, "player #{name} joined HUB"})
        # send(player_pid, {:broadcast, "line"})

        loop_zone(zone_data)

      {:broadcast, line} ->
        Logger.info "Attempt broadcast."
        player_list = Map.get(zone_data, :playerlist)

        for {k, player_pid} <- player_list do
          Logger.info "player name #{k}, player pid #{inspect(player_pid)}"
          send(player_pid, {:client_send, line})
        end

        loop_zone(zone_data)


      after 0 ->
        loop_zone(zone_data)
    end
  end
end
