defmodule ZoneManager do
  require Logger

  def start() do
    Logger.info("Starting up the zone manager.")
    Process.register(self(), :zone_manager)
    Logger.info("Zone Manager alias registered.")

    {:ok, hub_pid} =
      Task.Supervisor.start_child(Server.Services.ZoneManager, fn -> Zone.start("hub") end)

    zones = %{"hub" => hub_pid}

    loop_manager(zones)
  end

  def loop_manager(zones) do
    receive do
      {:player_join, username, pid} ->
        hub_pid = Map.get(zones, "hub")
        send(hub_pid, {:player_join, username, pid})
        loop_manager(zones)


      {:move_zone, zone, player_pid} ->
        Logger.info("Player-(#{player_pid}) requested Zone-(#{zone}) movement")
        loop_manager(zones)

      {:new_zone, zone_name} ->
        {:ok, zone_pid} =
          Task.Supervisor.start_child(Server.Services.ZoneManager, fn -> Zone.start(zone_name) end)

        zones = Map.put(zones, zone_name, zone_pid)
        loop_manager(zones)

      {:zones} ->
        Logger.info("#{zones}")
        loop_manager(zones)
    end
  end
end
