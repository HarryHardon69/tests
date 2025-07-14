using UnityEngine;

public class NoiseMain : MonoBehaviour
{
    private const int TextureSize = 256;

    public float IterationStep = 0.5f;

    private float TimerDelay = 0.05f;
    private float timer = 0;
    private Texture2D noiseTexture;
    private GameObject noiseTexObject;
    private GameObject debugText;
    private float iteration = 0;

    private Noise noise;
    public Noise.Types NoiseTypes = Noise.Types.Simplex;

    void Awake()
    {
        noiseTexture = new Texture2D(TextureSize, TextureSize, TextureFormat.RGBA32, false);
        noiseTexObject = GameObject.Find("NoiseTex");
        debugText = GameObject.Find("DebugText");
        noiseTexObject.guiTexture.texture = noiseTexture;
    }

    void Start()
    {
        noise = new Noise();
    }

    void Update()
    {
        timer -= Time.deltaTime;

        if (timer <= 0)
        {
            timer = timerDelay;

            var startTime = DateTime.Now;

            UpdateNoise();

        }
    }

    private void UpdateNoise()
    {
        noise.NoiseType = NoiseTypes;

        var startTime = DateTime.Now;

        var data = noise.GetValues(0, 0, iteration, TextureSize);

        var endTime = DateTime.Now;
        var elapsedTime = TimeSpan.FromTicks((endTime - startTime).Ticks);
        var time = string.Format("{0:D2}sec:{1:D2}msec", elapsedTime.Seconds, elapsedTime.Milliseconds);
        debugText.guiText.text = time;
        timerDelay = (elapsedTime.Seconds * 1000 + elapsedTime.Milliseconds) / 1000f;

        noiseTexture.SetPixels(data);
        noiseTexture.Apply();


        iteration += IterationStep;

    }
}
