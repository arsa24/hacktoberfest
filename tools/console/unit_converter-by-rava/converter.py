import argparse

CONVERSIONS = {
    "length": {
        "mm": 0.001, "cm": 0.01, "m": 1.0, "km": 1000.0,
        "inch": 0.0254, "ft": 0.3048, "yd": 0.9144, "mile": 1609.34,
    },
    "weight": {
        "mg": 0.001, "g": 1.0, "kg": 1000.0, "kwintal": 100000.0, "ton": 1000000.0,
        "lb": 453.592, "oz": 28.3495,
    },
    "volume": {
        "ml": 0.001, "liter": 1.0, "l": 1.0, "cc": 0.001, "m3": 1000.0, "gallon": 3.78541,
    },
    "area": {
        "m2": 1.0, "cm2": 0.0001, "km2": 1_000_000.0, "hektar": 10_000.0,
    },
    "temperature": {
        "C": "Celsius", "F": "Fahrenheit", "K": "Kelvin",
    }
}


def find_category(from_unit, to_unit):
    """Cari kategori otomatis dari dua unit."""
    for cat, units in CONVERSIONS.items():
        if from_unit in units and to_unit in units:
            return cat
    return None


def convert_length(value, from_unit, to_unit):
    return value * CONVERSIONS["length"][from_unit] / CONVERSIONS["length"][to_unit]


def convert_weight(value, from_unit, to_unit):
    return value * CONVERSIONS["weight"][from_unit] / CONVERSIONS["weight"][to_unit]


def convert_volume(value, from_unit, to_unit):
    return value * CONVERSIONS["volume"][from_unit] / CONVERSIONS["volume"][to_unit]


def convert_area(value, from_unit, to_unit):
    return value * CONVERSIONS["area"][from_unit] / CONVERSIONS["area"][to_unit]


def convert_temperature(value, from_unit, to_unit):
    # ke Celsius dulu
    if from_unit == "F":
        value = (value - 32) * 5 / 9
    elif from_unit == "K":
        value = value - 273.15
    # ke target
    if to_unit == "F":
        return (value * 9 / 5) + 32
    elif to_unit == "K":
        return value + 273.15
    return value


def convert(category, value, from_unit, to_unit):
    funcs = {
        "length": convert_length,
        "weight": convert_weight,
        "volume": convert_volume,
        "area": convert_area,
        "temperature": convert_temperature,
    }
    if category not in funcs:
        raise ValueError(f"Kategori '{category}' tidak dikenali.")
    return funcs[category](value, from_unit, to_unit)


def list_units():
    print("\n📏 Unit yang Didukung:")
    for cat, units in CONVERSIONS.items():
        print(f"- {cat.capitalize()}: {', '.join(units.keys())}")
    print()


def main():
    parser = argparse.ArgumentParser(description="🇮🇩 Unit Converter - Konversi satuan SI & umum di Indonesia")
    parser.add_argument("--value", type=float, help="Nilai yang akan dikonversi")
    parser.add_argument("--from", dest="from_unit", help="Satuan asal")
    parser.add_argument("--to", dest="to_unit", help="Satuan tujuan")
    parser.add_argument("--list", action="store_true", help="Tampilkan daftar unit yang tersedia")
    args = parser.parse_args()

    if args.list:
        list_units()
        return

    if not all([args.value, args.from_unit, args.to_unit]):
        print("Mode interaktif: Isi input di bawah\n")
        list_units()
        from_unit = input("Dari unit: ").strip()
        to_unit = input("Ke unit: ").strip()
        value = float(input("Masukkan nilai: "))
    else:
        from_unit, to_unit, value = args.from_unit, args.to_unit, args.value

    # auto detect kategori
    category = find_category(from_unit, to_unit)
    if not category:
        print(f"Gagal mengenali kategori untuk '{from_unit}' dan '{to_unit}'.")
        print("Gunakan --list untuk melihat unit yang valid.")
        return

    try:
        result = convert(category, value, from_unit, to_unit)
        print(f"\n {value} {from_unit} = {result:.4f} {to_unit}  ({category})\n")
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()
