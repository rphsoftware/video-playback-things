package space.rph.videoads;

import com.comphenix.protocol.ProtocolLibrary;
import com.comphenix.protocol.ProtocolManager;
import org.bukkit.entity.Player;
import org.bukkit.plugin.java.JavaPlugin;

import javax.imageio.ImageIO;
import java.io.File;
import java.io.IOException;
import java.lang.reflect.InvocationTargetException;

public final class Videoads extends JavaPlugin {
    public static ProtocolManager protocolManager;
    @Override
    public void onEnable() {
        protocolManager = ProtocolLibrary.getProtocolManager();
        State.pl = this;

        this.getCommand("image").setExecutor(new ImageCommand(this));
        this.getCommand("image2").setExecutor(new NewImageCommand());
        this.getCommand("image3").setExecutor(new Image3Command(this));
        this.getCommand("ae").setExecutor(new AECommand());
        getServer().getScheduler().scheduleSyncRepeatingTask(this, new Runnable() {
            @Override
            public void run() {
                if (State.running) {
                        for (int i = 0; i < State.mapCount; i++) {
                            try {
                                PacketStuff.prepareMapPacket(i);
                            } catch (IOException e) {
                                e.printStackTrace();
                            }
                        }
                }
            }
        }, 0, 1);
    }

    @Override
    public void onDisable() {
        // Plugin shutdown logic
    }
}
