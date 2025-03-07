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


    loop_zone(zone_data)
  end
end
