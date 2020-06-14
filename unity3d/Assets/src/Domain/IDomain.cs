using System.Collections.Generic;

namespace Domain
{
    public interface IDomain
    {
        void Execute();
        List<IEvent> TakeEvents();
    }
}