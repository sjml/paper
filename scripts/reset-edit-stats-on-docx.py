# so the template documents can always produce clean output

import os
import sys
import zipfile
import tempfile
import re
import shutil

if len(sys.argv) < 2:
    print("Enter path to a docx file")
    sys.exit(1)

cleaning_file = sys.argv[1]

with tempfile.TemporaryDirectory(dir=".") as tmpdir:
    zf = zipfile.ZipFile(cleaning_file)
    zf.extractall(tmpdir)

    core_file = os.path.join(tmpdir, "docProps", "core.xml")
    core_data = open(core_file, "r").read()
    core_fixed = re.sub(r"<cp:revision>(\d+)</cp:revision>", r"<cp:revision>0</cp:revision>", core_data)
    with open(core_file, "w") as core:
        core.write(core_fixed)

    app_file = os.path.join(tmpdir, "docProps", "app.xml")
    app_data = open(app_file, "r").read()
    app_fixed = re.sub(r"<TotalTime>(\d+)</TotalTime>", r"<TotalTime>0</TotalTime>", app_data)
    with open(app_file, "w") as app:
        app.write(app_fixed)

    shutil.make_archive(cleaning_file, "zip", tmpdir)
    shutil.move(f"{cleaning_file}.zip", cleaning_file)
