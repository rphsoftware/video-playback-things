package space.rph.videoads;

import org.bukkit.entity.Player;
import org.bukkit.map.MapCanvas;
import org.bukkit.map.MapRenderer;
import org.bukkit.map.MapView;

public class Renderer extends MapRenderer {
    @Override
    public void render(MapView mapView, MapCanvas mapCanvas, Player player) {
        for (int i = 0; i < mapCanvas.getCursors().size(); i++) {
            mapCanvas.getCursors().removeCursor(mapCanvas.getCursors().getCursor(i));
        }
    }

}
