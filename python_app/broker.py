#!/usr/bin/env python
import os
import json
import torch
from torchvision.models import vgg11
import albumentations as A
from albumentations.pytorch import ToTensorV2
from juans.utils.image import read_image


class Broker:
    def __init__(self):
        self.test_transforms = A.Compose([A.Resize(32, 32), A.Normalize(), ToTensorV2()])
        self.model = vgg11()
        self.model.eval()

    def broke(self, image_path="test.jpg"):
        # image_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), image_path)
        image_path = '/root/tmp_project/morpheus/python_app/test.jpg'
        image_data = read_image(image_path)
        one_batch = self.test_transforms(image=image_data)["image"].unsqueeze(0)
        with torch.no_grad():
            model_output = self.model(one_batch).squeeze().argmax()
        return json.dumps({
            "output": model_output.item()
        })

# broker = Broker()
# model_output = broker.broke(image_path="test.jpg")
# print(model_output)
