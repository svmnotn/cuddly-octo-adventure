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
    public static bool UnorderedEqual<T>(this ICollection<T> a, ICollection<T> b) {
      // Code taken from: http://www.dotnetperls.com/list-equals
      // 1: Require that the counts are equal
      if(a.Count != b.Count) {
        return false;
      }
      // 2: Initialize new Dictionary of the type
      Dictionary<T, int> d = new Dictionary<T, int>();
      // 3: Add each key's frequency from collection A to the Dictionary
      foreach(T item in a) {
        int c;
        if(d.TryGetValue(item, out c)) {
          d[item] = c + 1;
        } else {
          d.Add(item, 1);
        }
      }
      // 4: Add each key's frequency from collection B to the Dictionary
      // Return early if we detect a mismatch
      foreach(T item in b) {
        int c;
        if(d.TryGetValue(item, out c)) {
          if(c == 0) {
            return false;
          } else {
            d[item] = c - 1;
          }
        } else {
          // Not in dictionary
          return false;
        }
      }
      // 5: Verify that all frequencies are zero
      foreach(int v in d.Values) {
        if(v != 0) {
          return false;
        }
      }
      // 6: We know the collections are equal
      return true;
    }
  }
}