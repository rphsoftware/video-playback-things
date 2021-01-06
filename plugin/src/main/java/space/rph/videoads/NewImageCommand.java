package space.rph.videoads;

import com.destroystokyo.paper.Namespaced;
import com.google.common.collect.Multimap;
import net.md_5.bungee.api.chat.BaseComponent;
import org.bukkit.Bukkit;
import org.bukkit.Color;
import org.bukkit.Material;
import org.bukkit.attribute.Attribute;
import org.bukkit.attribute.AttributeModifier;
import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.enchantments.Enchantment;
import org.bukkit.entity.Player;
import org.bukkit.inventory.EquipmentSlot;
import org.bukkit.inventory.ItemFlag;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.ItemMeta;
import org.bukkit.inventory.meta.MapMeta;
import org.bukkit.inventory.meta.tags.CustomItemTagContainer;
import org.bukkit.map.MapView;
import org.bukkit.persistence.PersistentDataContainer;

import java.util.Collection;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class NewImageCommand implements CommandExecutor {

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        if (!(sender instanceof Player))
            return true;

        if (args.length != 1)
            return false;

        Player player = (Player) sender;

        // <amount of maps|int>

        int amount;

        try {
            amount = Integer.parseInt(args[0]);
        } catch(NumberFormatException e) {
            player.sendMessage("Invalid Number input!");
            return false;
        }

        State.mapCount = State.mapCount + amount;


        for (int i = 0; i < amount; i++) {
            ItemStack item = new ItemStack(Material.FILLED_MAP, 1);
            MapView map = Bukkit.createMap(Bukkit.getWorld("world"));

            MapMeta m = (MapMeta) item.getItemMeta();
            m.setMapView(map);
            item.setItemMeta(m);
            map.getRenderers().clear();
            map.addRenderer(new Renderer());
            player.getInventory().addItem(item);

        }

        return true;
    }
}
