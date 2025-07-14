namespace _Scripts.Noise
{
    public partial class Noise
    {
        public static float Value(float x, float y)
        {
            var i = x > 0 ? (int)x : (int)x - 1;
            var j = y > 0 ? (int)y : (int)y - 1;

            var ii = i & 255;
            var jj = j & 255;

            var nll = Perm[ii + Perm[jj]] / 255f;
            var nhl = Perm[ii + Perm[jj + 1]] / 255f;
            var nlh = Perm[ii + 1 + Perm[jj]] / 255f;
            var nhh = Perm[ii + 1 + Perm[jj + 1]] / 255f;

            var u = x - i;
            var v = y - j;

            //v1 + ((v2 - v1) * ratio)

            var nyl = nll + ((nhl - nll) * u); //nll, nhl, u;
            var nyh = nlh + ((nhh - nlh) * u); //nlh, nhh, u;

            var nxy = nyl + ((nyh - nyl) * v); //nyl, nyh, v;

            return nxy * 2 - 1f;
        }

        public static float Value(float x, float y, float z)
        {
            var i = x > 0 ? (int)x : (int)x - 1;
            var j = y > 0 ? (int)y : (int)y - 1;
            var k = z > 0 ? (int)z : (int)z - 1;

            var ii = i & 255;
            var jj = j & 255;
            var kk = k & 255;

            var nlll = Perm[ii + Perm[jj + Perm[kk]]] / 255f;
            var nlhl = Perm[ii + Perm[jj + 1 + Perm[kk]]] / 255f;
            var nhll = Perm[ii + 1 + Perm[jj + Perm[kk]]] / 255f;
            var nhhl = Perm[ii + 1 + Perm[jj + 1 + Perm[kk]]] / 255f;

            var nllh = Perm[ii + Perm[jj + Perm[kk + 1]]] / 255f;
            var nlhh = Perm[ii + Perm[jj + 1 + Perm[kk + 1]]] / 255f;
            var nhlh = Perm[ii + 1 + Perm[jj + Perm[kk + 1]]] / 255f;
            var nhhh = Perm[ii + 1 + Perm[jj + 1 + Perm[kk + 1]]] / 255f;

            var u = x - i;
            var v = y - j;
            var w = z - k;

            //v1 + ((v2 - v1) * ratio)

            var nyll = nlll + ((nhll - nlll) * u); //Mathf.Lerp(nlll, nhll, u);
            var nyhl = nlhl + ((nhhl - nlhl) * u); //Mathf.Lerp(nlhl, nhhl, u);
            var nylh = nllh + ((nhlh - nllh) * u); //Mathf.Lerp(nllh, nhlh, u);
            var nyhh = nlhh + ((nhhh - nlhh) * u); //Mathf.Lerp(nlhh, nhhh, u);

            var nxyl = nyll + ((nyhl - nyll) * v); //Mathf.Lerp(nyll, nyhl, v);
            var nxyh = nylh + ((nyhh - nylh) * v); //Mathf.Lerp(nylh, nyhh, v);

            var nxyz = nxyl + ((nxyh - nxyl) * w); //Mathf.Lerp(nxyl, nxyh, w);

            return nxyz * 2 - 1f;
        }
    }
}
