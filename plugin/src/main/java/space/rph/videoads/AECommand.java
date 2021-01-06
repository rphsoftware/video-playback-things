package space.rph.videoads;

import com.comphenix.protocol.PacketType;
import com.comphenix.protocol.events.PacketContainer;
import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Player;

import java.lang.reflect.InvocationTargetException;

public class AECommand implements CommandExecutor {

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        PacketContainer a = Videoads.protocolManager.createPacket(PacketType.Play.Server.GAME_STATE_CHANGE);
        a.getBytes()
                .write(0, (byte)5);
        a.getFloat()
                .write(0, 0f);

        if (!(sender instanceof Player))
            return true;

        for (Player p : State.pl.getServer().getOnlinePlayers()) {
            try {
                Videoads.protocolManager.sendServerPacket(p, a);
            } catch (InvocationTargetException e) {
                e.printStackTrace();
            }
        }
        return true;
    }
}
