defmodule Zone do
  require Logger

  def start(name) do
    Logger.info("Zone #{name} ready to recieve players.")

    player_list  = %{
      # Name, PID
    }

    zone_data = %{:zonename => "#{name}", :playerlist => player_list}

    loop_zone(zone_data)
  end

  def loop_zone(zone_data) do
    receive do
      {:player_join, name, player_pid} ->
        zonename = Map.get(zone_data, :zonename)
        Logger.info "New player #{inspect(name)} joined zone #{inspect(zone_data)}."
        player_list = Map.get(zone_data, :playerlist)
        player_list = Map.put(player_list, name, player_pid)
        zone_data = Map.put(zone_data, :playerlist, player_list)

        send(self(), {:broadcast, "Player #{name} joined HUB"})
        loop_zone(zone_data)

      {:broadcast, line} ->
        Logger.info "Attempt broadcast."
        player_list = Map.get(zone_data, :playerlist)

        for {k, player_pid} <- player_list do
          Logger.info "Player name #{k}, player pid #{inspect(player_pid)}"
          send(player_pid, {:client_send, line})
        end

        loop_zone(zone_data)


      after 0 ->
        loop_zone(zone_data)
    end
  end
end
