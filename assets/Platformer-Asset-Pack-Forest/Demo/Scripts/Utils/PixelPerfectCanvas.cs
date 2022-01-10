using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.U2D;
using UnityEngine.UI;

namespace AppleBoy
{
    public class PixelPerfectCanvas : MonoBehaviour
    {
        public PixelPerfectCamera pixelPerfectCamera;

        private CanvasScaler m_canvasScaler;

        private void Start()
        {
            m_canvasScaler = GetComponent<CanvasScaler>();
        }

        private void Update()
        {
            if (pixelPerfectCamera.stretchFill || !pixelPerfectCamera.enabled)
            {
                m_canvasScaler.uiScaleMode = CanvasScaler.ScaleMode.ScaleWithScreenSize;
                m_canvasScaler.referenceResolution = new Vector2(pixelPerfectCamera.refResolutionX, pixelPerfectCamera.refResolutionY);
            }
            else
            {
                m_canvasScaler.uiScaleMode = CanvasScaler.ScaleMode.ConstantPixelSize;
                m_canvasScaler.scaleFactor = pixelPerfectCamera.pixelRatio;
            }
        }
    }
}
