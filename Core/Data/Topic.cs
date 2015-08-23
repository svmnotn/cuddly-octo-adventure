namespace COA.Data {
  using System.Collections.Generic;

  public class Topic {
    public string name;
    public Dictionary<int, List<Question>> questions;

    public Question GetQuestion() {
      int[] tmp = new int[questions.Keys.Count];
      questions.Keys.CopyTo(tmp, 0);
      var keys = new List<int>(tmp);
      keys.Shuffle();
      return GetQuestion(keys[0]);
    }

    public Question GetQuestion(int val) {
      var qs = GetQuestions(val);
      Question tmp = null;
      if(qs.Count > 0) {
        tmp = qs[0];
      }
      return tmp;
    }
    
    public List<Question> GetQuestions(int val) {
      List<Question> qs;
      questions.TryGetValue(val, out qs);
      if(qs == null) {
        qs = new List<Question>();
        questions.Add(val, qs);
        return GetQuestions(val);
      }
      return qs;
    }

    public void Shuffle(int val) {
      var qs = GetQuestions(val);
      if(qs.Count > 0) {
        qs.Shuffle();
      }
    }
    
    public void ShuffleAll() {
      foreach(int key in questions.Keys) {
        Shuffle(key);
      }
    }

    public void AddQuestion(Question q) {
      GetQuestions(q.value).Add(q);
    }

    public bool RemoveQuestion(Question q) {
      return GetQuestions(q.value).Remove(q);
    }

    public void ChangeValue(Question q, int newVal) {
      RemoveQuestion(q);
      q.value = newVal;
      AddQuestion(q);
    }

    public Question GetQuestionFromID(string id) {
      foreach(var qList in questions.Values) {
        foreach(var q in qList) {
          if(q.id.Equals(id)) {
            return q;
          }
        }
      }
      return null;
    }

    public bool RemoveQuestionByID(string id) {
      var q = GetQuestionFromID(id);
      if(q != null){
          var qList = GetQuestions(q.value);
          return qList.Remove(q);
      }
      return false;
    }

    public bool HasQuestions() {
      int count = 0;
      foreach(int key in questions.Keys) {
        if(HasQuestions(key)) {
          count += 1;
        }
      }
      return count > 0;
    }

    public bool HasQuestions(int val) {
      var qs = GetQuestions(val);
      return (qs != null && qs.Count > 0);
    }

    public bool ContainsQuestion(Question q) {
      if(HasQuestions(q.value)) {
        return GetQuestions(q.value).Contains(q);
      }
      return false;
    }
  }
}