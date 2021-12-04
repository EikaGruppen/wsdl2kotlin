package dataclasses;

import java.util.ArrayList;
import java.util.List;

public class CxfMap {

    protected List<CxfMap.Entry> entry;

    public List<CxfMap.Entry> getEntry() {
        if (entry == null) {
            entry = new ArrayList<CxfMap.Entry>();
        }
        return this.entry;
    }

    public static class Entry {
         protected String key;
         protected Door value;
    }

}
