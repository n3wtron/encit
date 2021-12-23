import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IdentitySelectComponent } from './identity-select.component';

describe('IdentitySelectComponent', () => {
  let component: IdentitySelectComponent;
  let fixture: ComponentFixture<IdentitySelectComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ IdentitySelectComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(IdentitySelectComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
