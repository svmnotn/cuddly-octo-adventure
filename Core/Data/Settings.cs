﻿namespace COA.Data {
  using System;
  using System.Collections.Generic;
  using System.Drawing;

  public class Settings {
    public Font timerFont;
    public Color timerColor;
    public Color backgroundColor;
    [NonSerialized]
    public Image background;
    public string backgroundLoc;
    public SoundSettings sound;
    public TimeSettings time;
    public List<Team> teams;

    public static Settings Default {
      get {
        var tmp = new Settings();
        tmp.timerFont = SystemFonts.DefaultFont;
        tmp.timerColor = Color.Yellow;
        tmp.backgroundColor = Color.Silver;
        tmp.teams = new List<Team>();
        var t = new Team();
        t.name = "Blue Team";
        t.color = Color.Blue;
        t.font = SystemFonts.DefaultFont;
        tmp.teams.Add(t);
        t = new Team();
        t.name = "Red Team";
        t.color = Color.Red;
        t.font = SystemFonts.DefaultFont;
        tmp.teams.Add(t);
        tmp.sound = new SoundSettings();
        tmp.time = new TimeSettings();
        tmp.time.beforeAnswer = 60;
        tmp.time.afterAnswer = 5;
        tmp.time.scoreDelay = 5;
        tmp.time.gameOverDelay = 8;
        return tmp;
      }
    }
  }

  public class SoundSettings {
    public string backgroundSoundLoc;
    public string correctSoundLoc;
    public string wrongSoundLoc;
  }

  public class TimeSettings {
    public decimal beforeAnswer;
    public decimal afterAnswer;
    public decimal scoreDelay;
    public decimal gameOverDelay;
  }
}