using System;
using UnityEngine;

namespace _Scripts.Noise
{
    public partial class Noise
    {
        private const float F2 = 0.3660254f;
        private const float G2 = 0.2113249f;

        public static float Simplex(float x, float y)
        {
            float n0, n1, n2;

            var s = (x + y) * F2;
            var i = (x + s) > 0 ? (int)(x + s) : (int)(x + s) - 1;
            var j = (y + s) > 0 ? (int)(y + s) : (int)(y + s) - 1;

            var t = (i + j) * G2;

            var X0 = i - t;
            var Y0 = j - t;

            var x0 = x - X0;
            var y0 = y - Y0;

            int i1, j1;

            if (x0 > y0)
            {
                i1 = 1;
                j1 = 0;
            }
            else
            {
                i1 = 0;
                j1 = 1;
            }

            var x1 = x0 - i1 + G2;
            var y1 = y0 - j1 + G2;

            var x2 = x0 - 1.0f + 2.0f * G2;
            var y2 = y0 - 1.0f + 2.0f * G2;

            var ii = i & 255;
            var jj = j & 255;

            var gi0 = Perm[ii + Perm[jj]] % 12;
            var gi1 = Perm[ii + i1 + Perm[jj + j1]] % 12;
            var gi2 = Perm[ii + 1 + Perm[jj + 1]] % 12;

            var t0 = 0.5f - x0 * x0 - y0 * y0;
            if (t0 < 0)
                n0 = 0f;
            else
            {
                t0 *= t0;
                n0 = t0 * t0 * (Grad3[gi0].x * x0 * Grad3[gi0].y * y0); // Vector2.Dot(Grad3[gi0], new Vector2(x0, y0));
            }

            var t1 = 0.5f - x1 * x1 - y1 * y1;
            if (t1 < 0)
                n1 = 0f;
            else
            {
                t1 *= t1;
                n1 = t1 * t1 * (Grad3[gi1].x * x1 * Grad3[gi1].y * y1); // Vector2.Dot(Grad3[gi1], new Vector2(x1, y1));
            }

            var t2 = 0.5f - x2 * x2 - y2 * y2;
            if (t2 < 0)
                n2 = 0f;
            else
            {
                t2 *= t2;
                n2 = t2 * t2 * (Grad3[gi2].x * x2 * Grad3[gi2].y * y2); // Vector2.Dot(Grad3[gi2], new Vector2(x2, y2));
            }

            return 70.0f * (n0 + n1 + n2);
        }

        public static float Simplex(float x, float y, float z)
        {
            float n0, n1, n2, n3;

            var s = (x + y + z) * 0.3333333f;
            var i = (x + s) > 0 ? (int)(x + s) : (int)(x + s) - 1;
            var j = (y + s) > 0 ? (int)(y + s) : (int)(y + s) - 1;
            var k = (z + s) > 0 ? (int)(z + s) : (int)(z + s) - 1;

            var G3 = 0.1666667f;
            var t = (i + j + k) * G3;

            var X0 = i - t;
            var Y0 = j - t;
            var Z0 = k - t;

            var x0 = x - X0;
            var y0 = y - Y0;
            var z0 = z - Z0;

            int i1, j1, k1;
            int i2, j2, k2;

            if (x0 >= y0)
            {
                if (y0 >= z0)
                {
                    i1 = 1; j1 = 0; k1 = 0; i2 = 1; j2 = 1; k2 = 0;
                }
                else if (x0 >= z0)
                {
                    i1 = 1; j1 = 0; k1 = 0; i2 = 1; j2 = 0; k2 = 1;
                }
                else
                {
                    i1 = 0; j1 = 0; k1 = 1; i2 = 1; j2 = 0; k2 = 1;
                }
            }
            else
            {
                if (y0 < z0)
                {
                    i1 = 0; j1 = 0; k1 = 1; i2 = 0; j2 = 1; k2 = 1;
                }
                else if (x0 < z0)
                {
                    i1 = 0; j1 = 1; k1 = 0; i2 = 0; j2 = 1; k2 = 1;
                }
                else
                {
                    i1 = 0; j1 = 1; k1 = 0; i2 = 1; j2 = 1; k2 = 0;
                }
            }

            var x1 = x0 - i1 + G3;
            var y1 = y0 - j1 + G3;
            var z1 = z0 - k1 + G3;

            var x2 = x0 - i2 + 2.0f * G3;
            var y2 = y0 - j2 + 2.0f * G3;
            var z2 = z0 - k2 + 2.0f * G3;

            var x3 = x0 - 1.0f + 3.0f * G3;
            var y3 = y0 - 1.0f + 3.0f * G3;
            var z3 = z0 - 1.0f + 3.0f * G3;

            var ii = i & 255;
            var jj = j & 255;
            var kk = k & 255;

            var gi0 = Perm[ii + Perm[jj + Perm[kk]]] % 12;
            var gi1 = Perm[ii + i1 + Perm[jj + j1 + Perm[kk + k1]]] % 12;
            var gi2 = Perm[ii + i2 + Perm[jj + j2 + Perm[kk + k2]]] % 12;
            var gi3 = Perm[ii + 1 + Perm[jj + 1 + Perm[kk + 1]]] % 12;

            var t0 = 0.6f - x0 * x0 - y0 * y0 - z0 * z0;
            if (t0 < 0)
                n0 = 0f;
            else
            {
                t0 *= t0;
                n0 = t0 * t0 * (Grad3[gi0].x * x0 * Grad3[gi0].y * y0 + Grad3[gi0].z * z0); // Vector3.Dot(Grad3[gi0], new Vector3(x0, y0, z0));
            }

            var t1 = 0.6f - x1 * x1 - y1 * y1 - z1 * z1;
            if (t1 < 0)
                n1 = 0f;
            else
            {
                t1 *= t1;
                n1 = t1 * t1 * (Grad3[gi1].x * x1 * Grad3[gi1].y * y1 + Grad3[gi1].z * z1); // Vector3.Dot(Grad3[gi1], new Vector3(x1, y1, z1));
            }

            var t2 = 0.6f - x2 * x2 - y2 * y2 - z2 * z2;
            if (t2 < 0)
                n2 = 0f;
            else
            {
                t2 *= t2;
                n2 = t2 * t2 * (Grad3[gi2].x * x2 * Grad3[gi2].y * y2 + Grad3[gi2].z * z2); // Vector3.Dot(Grad3[gi2], new Vector3(x2, y2, z2));
            }

            var t3 = 0.6f - 3 * x3 - y3 * y3 - z3 * z3;
            if (t3 < 0)
                n3 = 0f;
            else
            {
                t3 *= t3;
                n2 = t3 * t3 * (Grad3[gi3].x * x3 * Grad3[gi3].y * y3 + Grad3[gi0].z * z3); //Vector3.Dot(Grad3[gi3], new Vector3(x3, y3, z3));
            }

            return 32.0f * (n0 + n1 + n2 + n3);
        }
    }
}
