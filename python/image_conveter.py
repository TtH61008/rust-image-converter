# pip install opencv-python numpy
import cv2
import os
import argparse
import numpy as np

parser = argparse.ArgumentParser()
parser.add_argument("--input-dir")
parser.add_argument("--output-dir")
args = parser.parse_args()

for basename in os.listdir(args.input_dir):
    if not basename.endswith(".png"):
        continue

    im = cv2.imread(f"{args.input_dir}/{basename}", -1)
    # 透過の場合は白埋めする
    if im.shape[-1]==4:
        index = np.where(im[:, :, 3] == 0)
        im[index] = [255, 255, 255, 255]
    im_gray = cv2.cvtColor(im, cv2.COLOR_BGR2GRAY)

    cv2.imwrite(f"{args.output_dir}/{basename}", im_gray)

