import time


def display_inventory(inventory):
    print("Inventory:")
    for item, count in inventory.items():
        print(f"{item}: {count}")
    total_items = sum(inventory.values())
    print(f"Total number of items: {total_items}")


def add_inventory(inventory, loot):
    for item in loot:
        inventory[item] = inventory.get(item, 0) + 1


start_time = time.time()

inventory = {"rope": 1, "torch": 6, "gold coin": 42, "dagger": 1, "arrow": 12}

dragon_loot = ["gold coin", "dagger", "gold coin", "gold coin", "ruby"]

display_inventory(inventory)
add_inventory(inventory, dragon_loot)
display_inventory(inventory)

elapsed_time = time.time() - start_time
print(f"Time taken: {elapsed_time:.6f} seconds")
