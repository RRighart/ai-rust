pip install tf2onnx

import tensorflow as tf
import tf2onnx
from tf2onnx.convert import from_keras
import os


def convert_to_onnx(sourcedir, outputdir):
    """
    sourcedir: path to file hdf5 or h5 file.
    outputdir: path to the newly created onnx file.
    """
    model = tf.keras.models.load_model(sourcedir)
    onnx_model, _ = from_keras(model)
    
    print("starting conversion")
    try:
        with open(outputdir, 'wb') as f:
            f.write(onnx_model.SerializeToString())
    except:
        print("conversion did not work")

convert_to_onnx("color-model.pb", "color-model-new.onnx")

convert_to_onnx("/home/jovyan/BT2023/scripts/CNN-bathtub-detector.h5", "converted_bathtub_detector.onnx")

