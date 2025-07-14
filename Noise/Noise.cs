using UnityEngine;

namespace _Scripts.Noise
{
    public partial class Noise
    {
        public enum Types {Value, Perlin, Simplex}

        private static bool isInitialized;

        public Types NoiseType { get; set; }
        public int Octaves { get; set; }
        public float Frequency { get; set; }
        public float Lacunarity { get; set; }
        public float Persistence { get; set; }
        public float Gain { get; set; }

        private static readonly Vector3[] Grad3 = {
                                                        new Vector3(1, 1, 0), new Vector3(-1, 1, 0), new Vector3(1, -1, 0), new Vector3(-1, -1, 0),
                                                        new Vector3(1, 0, 1), new Vector3(-1, 0, 1), new Vector3(1, 0, -1), new Vector3(-1, 0, -1),
                                                        new Vector3(0, 1, 1), new Vector3(0, -1, 1), new Vector3(0, 1, -1), new Vector3(0, -1, -1),
                                                  };

        private static readonly int[] P = {
                                                151, 160, 137,  91,  90,  15, 131,  13, 201,  95,  96,  53, 194, 223,   7, 225,   // Number 0 - 255 in random order
                                                140,  36, 103,  30,  69, 142,   8,  99,  37, 240,  21,  10,  23, 190,   6, 148,
                                                247, 120, 234,  75,   0,  26, 197,  62,  94, 252, 219, 203, 117,  35,  11,  32,
                                                 57, 177,  33,  88, 237, 149,  56,  87, 174,  20, 125, 136, 171, 168,  68, 175,
                                                 74, 165,  71, 134, 139,  48,  27, 166,  77, 146, 158, 231,  83, 111, 229, 122,
                                                 60, 211, 133, 230, 220, 105,  92,  41,  55,  46, 245,  40, 244, 102, 143,  54,
                                                 65,  25,  63, 161,   1, 216,  80,  73, 209,  76, 132, 187, 208,  89,  18, 169,
                                                200, 196, 135, 130, 116, 188, 159,  86, 164, 100, 109, 198, 173, 186,   3,  64,
                                                 52, 217, 226, 250, 124, 123,   5, 202,  38, 147, 118, 126, 255,  82,  85, 212,
                                                207, 206,  59, 227,  47,  16,  58,  17, 182, 189,  28,  42, 223, 183, 170, 213,
                                                119, 248, 152,   2,  44, 154, 163,  70, 221, 153, 101, 155, 167,  43, 172,   9,
                                                129,  22,  39, 253,  19,  98, 108, 110,  79, 113, 224, 232, 178, 185, 112, 104,
                                                218, 246,  97, 228, 251,  34, 242, 193, 238, 210, 144,  12, 191, 179, 162, 241,
                                                 81,  51, 145, 235, 249,  14, 239, 107,  49, 192, 214,  31, 181, 199, 106, 157,
                                                184,  84, 204, 176, 115, 121,  50,  45, 127,   4, 150, 254, 138, 236, 205,  93,
                                                222, 114,  67,  29,  24,  72, 243, 141, 128, 195,  78,  66, 215,  61, 156, 180

                                          };

        private static readonly int[] Perm = new int[512];

        public Noise()
        {
            if (!isInitialized)
                for (int i = 0; i < Perm.Length; i++)
                    Perm[i] = P[i & 255];

            NoiseType = Types.Simplex;
            Octaves = 4;
            Frequency = 0.2f;
            Lacunarity = 1.9f;
            Persistence = 1.8f;
            Gain = 0.33f;

            isInitialized = true;
        }

        public Noise(Types noiseType, int octaves, float frequency, float lacunarity, float persistence, float gain) : this()
        {
            NoiseType = noiseType;
            Octaves = octaves;
            Frequency = frequency;
            Lacunarity = lacunarity;
            Persistence = persistence;
            Gain = gain;
        }
    }

    public Color GetValue(float xIn, float yIn, bool useAlpha)
    {
        var noise = 0f;
        var signal = 0f;
        var persistence = 1f;

        xIn = (xIn + 1000000) * Frequency;
        yIn = (yIn + 1000000) * Frequency;

        for (int i = 0; i < Octaves; i++)
        {
            if (NoiseType == Types.Value)
                signal = (Value(xIn, yIn) + 1) / 2f;
            else if (NoiseType == Types.Perlin)
                signal = (Perlin(xIn, yIn) + 1) / 2f;
            else if (NoiseType == Types.Simplex)
                signal = (Simplex(xIn, yIn) + 1) / 2f;

            noise += signal * persistence;

            xIn /= Lacunarity;
            yIn /= Lacunarity;
            persistence *= Persistence;
        }

        noise = noise / Octaves * Gain;

        if (useAlpha)
            return new Color(noise, noise, noise, noise);
        else
        {
           return new Color (noise, noise, noise, 1);
        }
    }

    public Color GetValue(float xIn, float yIn, float zIn, bool useAlpha)
    {
        var noise = 0f;
        var signal = 0f;
        var persistence = 1f;

        xIn = (xIn + 1000000) * Frequency;
        yIn = (yIn + 1000000) * Frequency;
        zIn = (zIn + 1000000) * Frequency;

        for (int i = 0; i < Octaves; i++)
        {
            if (NoiseType == Types.Value)
                signal = (Value(xIn, yIn, zIn) + 1) / 2f;
            else if (NoiseType == Types.Perlin)
                signal = (Perlin(xIn, yIn, zIn) + 1) / 2f;
            else if (NoiseType == Types.Simplex)
                signal = (Simplex(xIn, yIn, zIn) + 1) / 2f;

            noise += signal * persistence;

            xIn /= Lacunarity;
            yIn /= Lacunarity;
            zIn /= Lacunarity;
            persistence *= Persistence;
        }

        noise = noise / Octaves * Gain;

        if (useAlpha)
            return new Color(noise, noise, noise, noise);
        else
        {
           return new Color (noise, noise, noise, 1);
        }
    }

    public Color [] GetValues(float xIn, float yIn, int size, bool useAlpha)
    {
        var data = new Color[size * size];

        var counter = 0;

        for (int y = (int)yIn; y < (int)yIn + size; y++)
            for (int x = (int)xIn; x < (int)xIn + size; x++)
            {
                data[counter] = GetValue(x, y, useAlpha);
                counter++;
            }

        return data;
    }

    public Color [] GetValues(float xIn, float yIn, fkiat zIn, int size, bool useAlpha)
    {
        var data = new Color[size * size];

        var counter = 0;

        for (int y = (int)yIn; y < (int)yIn + size; y++)
            for (int x = (int)xIn; x < (int)xIn + size; x++)
            {
                data[counter] = GetValue(x, y, zIn, useAlpha);
                counter++;
            }

        return data;
    }
}
