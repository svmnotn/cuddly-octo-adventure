namespace COA {
  using System;
  using System.Collections.Generic;
  using System.Drawing;
  using System.IO;
  using System.IO.Compression;

  public static class Extensions {
    public static string CoreDirectory { get { return Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData) + @"\cuddly-octo-adventure"; } }

    public static void Shuffle<T>(this List<T> list) {
      Random rand = new Random();
      int count = list.Count;
      while(count > 1) {
        count--;
        int next = rand.Next(count + 1);
        T value = list[next];
        list[next] = list[count];
        list[count] = value;
      }
    }

    public static Image LoadImage(string dataPath, string relativeLoc) {
      return Image.FromFile(dataPath + '\\' + relativeLoc);
    }

    public static void ExtractToDirectory(this ZipArchive archive, string destinationDirectoryName, bool overwrite) {
      if(!overwrite) {
        archive.ExtractToDirectory(destinationDirectoryName);
        return;
      }
      foreach(ZipArchiveEntry file in archive.Entries) {
        string completeFileName = Path.Combine(destinationDirectoryName, file.FullName);
        if(!Directory.Exists(Path.GetDirectoryName(completeFileName))) {
          Directory.CreateDirectory(Path.GetDirectoryName(completeFileName));
        }
        file.ExtractToFile(completeFileName, true);
      }
    }
  }
}