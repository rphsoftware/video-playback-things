package space.rph.videoads;

import com.comphenix.protocol.PacketType;
import com.comphenix.protocol.events.PacketContainer;
import org.bukkit.entity.Player;

import java.io.*;
import java.lang.reflect.InvocationTargetException;
import java.net.MalformedURLException;
import java.net.URL;

public class PacketStuff {
    public static PacketContainer prepareMapPacket(int mid) throws IOException {
        PacketContainer a = Videoads.protocolManager.createPacket(PacketType.Play.Server.MAP);

        URL url = new URL("http://localhost:5050/" + mid);
        InputStream in = new BufferedInputStream(url.openStream());
        byte[] frog = new byte[128*128];
        int ea = in.read(frog);
        in.close();

        a.getIntegers()
                .write(0, mid + State.startMapId)
                .write(1, 0)
                .write(2, 0)
                .write(3, 128)
                .write(4, 128);

        a.getBytes()
                .write(0, (byte) 0);

        a.getBooleans()
                .write(0, true)
                .write(1, false);

        a.getByteArrays()
                .write(0, frog);

        for (Player p : State.pl.getServer().getOnlinePlayers()) {
            try {
                Videoads.protocolManager.sendServerPacket(p, a);
            } catch (InvocationTargetException e) {
                e.printStackTrace();
            }
        }

        return a;
    }
}
