using UnityEngine;

namespace _Scripts.Noise
{
    public partial class Noise
    {
        public static float Perlin(float x, float y)
        {
            var i = x > 0 ? (int)x : (int)x - 1;
            var j = y > 0 ? (int)y : (int)y - 1;

            x = x - i;
            y = y - j;

            i = i & 255;
            j = j & 255;

            var gll = Perm[i + Perm[j]] % 12;
            var glh = Perm[i + Perm[j + 1]] % 12;
            var ghl = Perm[i + 1 + Perm[j]] % 12;
            var ghh = Perm[i + 1 + Perm[j + 1]] % 12;

            //dot = x1 * x2 + y1 * y2

            var nll = Grad3[gll].x * x + Grad3[gll].y * y; //Vector2.Dot(Grad3[gll], new Vector2(x, y));
            var nlh = Grad3[glh].x * x + Grad3[glh].y * (y - 1); //Vector2.Dot(Grad3[glh], new Vector2(x, y - 1));
            var nhl = Grad3[ghl].x * (x - 1) + Grad3[ghl].y * y; //Vector2.Dot(Grad3[ghl], new Vector2(x - 1, y));
            var nhh = Grad3[ghh].x * (x - 1) + Grad3[ghh].y * (y - 1); //Vector2.Dot(Grad3[ghh], new Vector2(x - 1, y - 1));

            var u = x * x * x * (x *(x * 6 - 15) + 10);
            var v = y * y * y * (y *(y * 6 - 15) + 10);

            //v1 + ((v2 - v1) * ratio)

            var nyl = nll + ((nhl - nll) * u); //Mathf.Lerp(nll, nhl, u);
            var nyh = nlh + ((nhh - nlh) * u); //Mathf.Lerp(nlh, nhh, u);

            var nxy = nyl + ((nyh - nyl) * v); //Mathf.Lerp(nyl, nyh, v);

            return nxy;
        }

        public static float Perlin(float x, float y, float z)
        {
            var i = x > 0 ? (int)x : (int)x - 1;
            var j = y > 0 ? (int)y : (int)y - 1;
            var k = z > 0 ? (int)z : (int)z - 1;

            x = x - i;
            y = y - j;
            z = z - k;

            i = i & 255;
            j = j & 255;
            k = k & 255;

            var glll = Perm[i + Perm[j + Perm[k]]] % 12;
            var glhl = Perm[i + Perm[j + 1 + Perm[k]]] % 12;
            var ghll = Perm[i + 1 + Perm[j + Perm[k]]] % 12;
            var ghhl = Perm[i + 1 + Perm[j + 1 + Perm[k]]] % 12;
            var gllh = Perm[i + Perm[j + Perm[k + 1]]] % 12;
            var glhh = Perm[i + Perm[j + 1 + Perm[k + 1]]] % 12;
            var ghlh = Perm[i + 1 + Perm[j + Perm[k + 1]]] % 12;
            var ghhh = Perm[i + 1 + Perm[j + 1 + Perm[k + 1]]] % 12;

            //dot = x1 * x2 + y1 * y2 + z1 * z2

            var nlll = Grad3[glll].x * x + Grad3[glll].y * y + Grad3[glll].z * z; //Vector3.Dot(Grad3[glll], new Vector3(x, y, z));
            var nlhl = Grad3[glhl].x * x + Grad3[glhl].y * (y - 1) + Grad3[glhl].z * z; //Vector3.Dot(Grad3[glhl], new Vector3(x, y - 1, z));
            var nhll = Grad3[ghll].x * (x - 1) + Grad3[ghll].y * y + Grad3[ghll].z * z; //Vector3.Dot(Grad3[ghll], new Vector3(x - 1, y, z));
            var nhhl = Grad3[ghhl].x * (x - 1) + Grad3[ghhl].y * (y- 1) + Grad3[ghhl].z * z; //Vector3.Dot(Grad3[ghhl], new Vector3(x - 1, y - 1, z));
            var nllh = Grad3[gllh].x * x + Grad3[gllh].y * y + Grad3[gllh].z * (z - 1); //Vector3.Dot(Grad3[gllh], new Vector3(x, y, z - 1));
            var nlhh = Grad3[glhh].x * x + Grad3[glhh].y * (y - 1) + Grad3[glhh].z * (z - 1); //Vector3.Dot(Grad3[glhh], new Vector3(x, y - 1, z - 1));
            var nhlh = Grad3[ghlh].x * (x - 1) + Grad3[ghlh].y * y + Grad3[ghlh].z * (z - 1); //Vector3.Dot(Grad3[ghlh], new Vector3(x - 1, y, z - 1));
            var nhhh = Grad3[ghhh].x * (x - 1) + Grad3[ghhh].y * (y - 1) + Grad3[ghhh].z * (z - 1); //Vector3.Dot(Grad3[ghhh], new Vector3(x - 1, y - 1, z - 1));

            var u = x * x * x * (x *(x * 6 - 15) + 10);
            var v = y * y * y * (y *(y * 6 - 15) + 10);
            var w = z * z * z * (z *(z * 6 - 15) + 10);

            var nxll = nlll + ((nhll - nlll) * u); //Mathf.Lerp(nlll, nhll, u);
            var nxlh = nllh + ((nhlh - nllh) * u); //Mathf.Lerp(nllh, nhlh, u);
            var nxhl = nlhl + ((nhhl - nlhl) * u); //Mathf.Lerp(nlhl, nhhl, u);
            var nxhh = nlhh + ((nhhh - nlhh) * u); //Mathf.Lerp(nlhh, nhhh, u);

            var nxyl = nxll + ((nxhl - nxll) * v); //Mathf.Lerp(nxll, nxhl, v);
            var nxyh = nxlh + ((nxhh - nxlh) * v); //Mathf.Lerp(nxlh, nxhh, v);

            var nxyz = nxyl + ((nxyh - nxyl) * w); //Mathf.Lerp(nxyl, nxyh, w);

            return nxyz;
        }
    }
}
